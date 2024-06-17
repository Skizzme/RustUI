
        mod __gl_imports {
            pub use std::mem;
            pub use std::os::raw;
        }
    

        #[inline(never)]
        fn metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                      symbol: &'static str,
                      fallbacks: &[&'static str]) -> *const __gl_imports::raw::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in fallbacks {
                    ptr = loadfn(sym);
                    if !ptr.is_null() { break; }
                }
            }
            ptr
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum,
                                                 gltype: GLenum,
                                                 id: GLuint,
                                                 severity: GLenum,
                                                 length: GLsizei,
                                                 message: *const GLchar,
                                                 userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint,
                                                    category: GLenum,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;


        }
    
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM: types::GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_ALPHA_BITS: types::GLenum = 0x0D5B;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_BLUE_BITS: types::GLenum = 0x0D5A;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_BUFFER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_CLEAR_VALUE: types::GLenum = 0x0B80;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_GREEN_BITS: types::GLenum = 0x0D59;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_RED_BITS: types::GLenum = 0x0D58;
#[allow(dead_code, non_upper_case_globals)] pub const ADD: types::GLenum = 0x0104;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA12: types::GLenum = 0x803D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA16: types::GLenum = 0x803E;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA4: types::GLenum = 0x803B;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA8: types::GLenum = 0x803C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BIAS: types::GLenum = 0x0D1D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BITS: types::GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_SCALE: types::GLenum = 0x0D1C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST: types::GLenum = 0x0BC0;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_FUNC: types::GLenum = 0x0BC1;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_REF: types::GLenum = 0x0BC2;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT: types::GLenum = 0x1200;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT_AND_DIFFUSE: types::GLenum = 0x1602;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB0;
#[allow(dead_code, non_upper_case_globals)] pub const AUTO_NORMAL: types::GLenum = 0x0D80;
#[allow(dead_code, non_upper_case_globals)] pub const AUX0: types::GLenum = 0x0409;
#[allow(dead_code, non_upper_case_globals)] pub const AUX1: types::GLenum = 0x040A;
#[allow(dead_code, non_upper_case_globals)] pub const AUX2: types::GLenum = 0x040B;
#[allow(dead_code, non_upper_case_globals)] pub const AUX3: types::GLenum = 0x040C;
#[allow(dead_code, non_upper_case_globals)] pub const AUX_BUFFERS: types::GLenum = 0x0C00;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BITMAP: types::GLenum = 0x1A00;
#[allow(dead_code, non_upper_case_globals)] pub const BITMAP_TOKEN: types::GLenum = 0x0704;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BIAS: types::GLenum = 0x0D1B;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BITS: types::GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_SCALE: types::GLenum = 0x0D1A;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const C3F_V3F: types::GLenum = 0x2A24;
#[allow(dead_code, non_upper_case_globals)] pub const C4F_N3F_V3F: types::GLenum = 0x2A26;
#[allow(dead_code, non_upper_case_globals)] pub const C4UB_V2F: types::GLenum = 0x2A22;
#[allow(dead_code, non_upper_case_globals)] pub const C4UB_V3F: types::GLenum = 0x2A23;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP: types::GLenum = 0x2900;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB1;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_PIXEL_STORE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_VERTEX_ARRAY_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const COEFF: types::GLenum = 0x0A00;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY: types::GLenum = 0x8076;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_POINTER: types::GLenum = 0x8090;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_SIZE: types::GLenum = 0x8081;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_STRIDE: types::GLenum = 0x8083;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_TYPE: types::GLenum = 0x8082;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_INDEX: types::GLenum = 0x1900;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_INDEXES: types::GLenum = 0x1603;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL: types::GLenum = 0x0B57;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL_FACE: types::GLenum = 0x0B55;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL_PARAMETER: types::GLenum = 0x0B56;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE: types::GLenum = 0x1300;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_AND_EXECUTE: types::GLenum = 0x1301;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ATTENUATION: types::GLenum = 0x1207;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_PIXEL_TOKEN: types::GLenum = 0x0706;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_COLOR: types::GLenum = 0x0B00;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_INDEX: types::GLenum = 0x0B01;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_NORMAL: types::GLenum = 0x0B02;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_COLOR: types::GLenum = 0x0B04;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_DISTANCE: types::GLenum = 0x0B09;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_INDEX: types::GLenum = 0x0B05;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_POSITION: types::GLenum = 0x0B07;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_POSITION_VALID: types::GLenum = 0x0B08;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_TEXTURE_COORDS: types::GLenum = 0x0B06;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DECAL: types::GLenum = 0x2101;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BIAS: types::GLenum = 0x0D1F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BITS: types::GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_SCALE: types::GLenum = 0x0D1E;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DIFFUSE: types::GLenum = 0x1201;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DOMAIN: types::GLenum = 0x0A02;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_PIXEL_TOKEN: types::GLenum = 0x0705;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG: types::GLenum = 0x0B43;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY: types::GLenum = 0x8079;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY_POINTER: types::GLenum = 0x8093;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY_STRIDE: types::GLenum = 0x808C;
#[allow(dead_code, non_upper_case_globals)] pub const EMISSION: types::GLenum = 0x1600;
#[allow(dead_code, non_upper_case_globals)] pub const ENABLE_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EVAL_BIT: types::GLenum = 0x00010000;
#[allow(dead_code, non_upper_case_globals)] pub const EXP: types::GLenum = 0x0800;
#[allow(dead_code, non_upper_case_globals)] pub const EXP2: types::GLenum = 0x0801;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const EYE_LINEAR: types::GLenum = 0x2400;
#[allow(dead_code, non_upper_case_globals)] pub const EYE_PLANE: types::GLenum = 0x2502;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK: types::GLenum = 0x1C01;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_POINTER: types::GLenum = 0x0DF0;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_SIZE: types::GLenum = 0x0DF1;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_TYPE: types::GLenum = 0x0DF2;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FLAT: types::GLenum = 0x1D00;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FOG: types::GLenum = 0x0B60;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_COLOR: types::GLenum = 0x0B66;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_DENSITY: types::GLenum = 0x0B62;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_END: types::GLenum = 0x0B64;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_HINT: types::GLenum = 0x0C54;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_INDEX: types::GLenum = 0x0B61;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_MODE: types::GLenum = 0x0B65;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_START: types::GLenum = 0x0B63;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BIAS: types::GLenum = 0x0D19;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BITS: types::GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_SCALE: types::GLenum = 0x0D18;
#[allow(dead_code, non_upper_case_globals)] pub const HINT_BIT: types::GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY: types::GLenum = 0x8077;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_POINTER: types::GLenum = 0x8091;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_STRIDE: types::GLenum = 0x8086;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_TYPE: types::GLenum = 0x8085;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_BITS: types::GLenum = 0x0D51;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_CLEAR_VALUE: types::GLenum = 0x0C20;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_LOGIC_OP: types::GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_MODE: types::GLenum = 0x0C30;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_OFFSET: types::GLenum = 0x0D13;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_SHIFT: types::GLenum = 0x0D12;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_WRITEMASK: types::GLenum = 0x0C21;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY: types::GLenum = 0x8049;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY12: types::GLenum = 0x804C;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY16: types::GLenum = 0x804D;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY4: types::GLenum = 0x804A;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY8: types::GLenum = 0x804B;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT0: types::GLenum = 0x4000;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT1: types::GLenum = 0x4001;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT2: types::GLenum = 0x4002;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT3: types::GLenum = 0x4003;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT4: types::GLenum = 0x4004;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT5: types::GLenum = 0x4005;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT6: types::GLenum = 0x4006;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT7: types::GLenum = 0x4007;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTING: types::GLenum = 0x0B50;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTING_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_LOCAL_VIEWER: types::GLenum = 0x0B51;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_ATTENUATION: types::GLenum = 0x1208;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_RESET_TOKEN: types::GLenum = 0x0707;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE: types::GLenum = 0x0B24;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE_PATTERN: types::GLenum = 0x0B25;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE_REPEAT: types::GLenum = 0x0B26;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_TOKEN: types::GLenum = 0x0702;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_BASE: types::GLenum = 0x0B32;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_BIT: types::GLenum = 0x00020000;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_INDEX: types::GLenum = 0x0B33;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_MODE: types::GLenum = 0x0B30;
#[allow(dead_code, non_upper_case_globals)] pub const LOAD: types::GLenum = 0x0101;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP: types::GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE: types::GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12: types::GLenum = 0x8041;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12_ALPHA12: types::GLenum = 0x8047;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12_ALPHA4: types::GLenum = 0x8046;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE16: types::GLenum = 0x8042;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE16_ALPHA16: types::GLenum = 0x8048;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE4: types::GLenum = 0x803F;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE4_ALPHA4: types::GLenum = 0x8043;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE6_ALPHA2: types::GLenum = 0x8044;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE8: types::GLenum = 0x8040;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE8_ALPHA8: types::GLenum = 0x8045;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_ALPHA: types::GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_COLOR_4: types::GLenum = 0x0D90;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_GRID_DOMAIN: types::GLenum = 0x0DD0;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_GRID_SEGMENTS: types::GLenum = 0x0DD1;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_INDEX: types::GLenum = 0x0D91;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_NORMAL: types::GLenum = 0x0D92;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_1: types::GLenum = 0x0D93;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_2: types::GLenum = 0x0D94;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_3: types::GLenum = 0x0D95;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_4: types::GLenum = 0x0D96;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_VERTEX_3: types::GLenum = 0x0D97;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_VERTEX_4: types::GLenum = 0x0D98;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_COLOR_4: types::GLenum = 0x0DB0;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_GRID_DOMAIN: types::GLenum = 0x0DD2;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_GRID_SEGMENTS: types::GLenum = 0x0DD3;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_INDEX: types::GLenum = 0x0DB1;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_NORMAL: types::GLenum = 0x0DB2;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_1: types::GLenum = 0x0DB3;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_2: types::GLenum = 0x0DB4;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_3: types::GLenum = 0x0DB5;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_4: types::GLenum = 0x0DB6;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_VERTEX_3: types::GLenum = 0x0DB7;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_VERTEX_4: types::GLenum = 0x0DB8;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_COLOR: types::GLenum = 0x0D10;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_STENCIL: types::GLenum = 0x0D11;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_MODE: types::GLenum = 0x0BA0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D35;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D3B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_PLANES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_EVAL_ORDER: types::GLenum = 0x0D30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LIGHTS: types::GLenum = 0x0D31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LIST_NESTING: types::GLenum = 0x0B31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_STACK_DEPTH: types::GLenum = 0x0D37;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PIXEL_MAP_TABLE: types::GLenum = 0x0D34;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROJECTION_STACK_DEPTH: types::GLenum = 0x0D38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_STACK_DEPTH: types::GLenum = 0x0D39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW: types::GLenum = 0x1700;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_MATRIX: types::GLenum = 0x0BA6;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_STACK_DEPTH: types::GLenum = 0x0BA3;
#[allow(dead_code, non_upper_case_globals)] pub const MODULATE: types::GLenum = 0x2100;
#[allow(dead_code, non_upper_case_globals)] pub const MULT: types::GLenum = 0x0103;
#[allow(dead_code, non_upper_case_globals)] pub const N3F_V3F: types::GLenum = 0x2A25;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_STACK_DEPTH: types::GLenum = 0x0D70;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NORMALIZE: types::GLenum = 0x0BA1;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY: types::GLenum = 0x8075;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_POINTER: types::GLenum = 0x808F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_STRIDE: types::GLenum = 0x807F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_TYPE: types::GLenum = 0x807E;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_LINEAR: types::GLenum = 0x2401;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_PLANE: types::GLenum = 0x2501;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const ORDER: types::GLenum = 0x0A01;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)] pub const PASS_THROUGH_TOKEN: types::GLenum = 0x0700;
#[allow(dead_code, non_upper_case_globals)] pub const PERSPECTIVE_CORRECTION_HINT: types::GLenum = 0x0C50;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_A_TO_A: types::GLenum = 0x0C79;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_A_TO_A_SIZE: types::GLenum = 0x0CB9;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_B_TO_B: types::GLenum = 0x0C78;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_B_TO_B_SIZE: types::GLenum = 0x0CB8;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_G_TO_G: types::GLenum = 0x0C77;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_G_TO_G_SIZE: types::GLenum = 0x0CB7;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_A: types::GLenum = 0x0C75;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_A_SIZE: types::GLenum = 0x0CB5;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_B: types::GLenum = 0x0C74;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_B_SIZE: types::GLenum = 0x0CB4;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_G: types::GLenum = 0x0C73;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_G_SIZE: types::GLenum = 0x0CB3;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_I: types::GLenum = 0x0C70;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_I_SIZE: types::GLenum = 0x0CB0;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_R: types::GLenum = 0x0C72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_R_SIZE: types::GLenum = 0x0CB2;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_R_TO_R: types::GLenum = 0x0C76;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_R_TO_R_SIZE: types::GLenum = 0x0CB6;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_S_TO_S: types::GLenum = 0x0C71;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_S_TO_S_SIZE: types::GLenum = 0x0CB1;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MODE_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH: types::GLenum = 0x0B10;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH_HINT: types::GLenum = 0x0C51;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_TOKEN: types::GLenum = 0x0701;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON: types::GLenum = 0x0009;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_STIPPLE: types::GLenum = 0x0B42;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_STIPPLE_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_TOKEN: types::GLenum = 0x0703;
#[allow(dead_code, non_upper_case_globals)] pub const POSITION: types::GLenum = 0x1203;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION: types::GLenum = 0x1701;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_MATRIX: types::GLenum = 0x0BA7;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_STACK_DEPTH: types::GLenum = 0x0BA4;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const Q: types::GLenum = 0x2003;
#[allow(dead_code, non_upper_case_globals)] pub const QUADRATIC_ATTENUATION: types::GLenum = 0x1209;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUAD_STRIP: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const R: types::GLenum = 0x2002;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BIAS: types::GLenum = 0x0D15;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BITS: types::GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)] pub const RED_SCALE: types::GLenum = 0x0D14;
#[allow(dead_code, non_upper_case_globals)] pub const RENDER: types::GLenum = 0x1C00;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const RENDER_MODE: types::GLenum = 0x0C40;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RETURN: types::GLenum = 0x0102;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_MODE: types::GLenum = 0x0C31;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const S: types::GLenum = 0x2000;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BIT: types::GLenum = 0x00080000;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SELECT: types::GLenum = 0x1C02;
#[allow(dead_code, non_upper_case_globals)] pub const SELECTION_BUFFER_POINTER: types::GLenum = 0x0DF3;
#[allow(dead_code, non_upper_case_globals)] pub const SELECTION_BUFFER_SIZE: types::GLenum = 0x0DF4;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADE_MODEL: types::GLenum = 0x0B54;
#[allow(dead_code, non_upper_case_globals)] pub const SHININESS: types::GLenum = 0x1601;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH: types::GLenum = 0x1D01;
#[allow(dead_code, non_upper_case_globals)] pub const SPECULAR: types::GLenum = 0x1202;
#[allow(dead_code, non_upper_case_globals)] pub const SPHERE_MAP: types::GLenum = 0x2402;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_CUTOFF: types::GLenum = 0x1206;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_DIRECTION: types::GLenum = 0x1204;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_EXPONENT: types::GLenum = 0x1205;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BITS: types::GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const T: types::GLenum = 0x2001;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C3F_V3F: types::GLenum = 0x2A2A;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C4F_N3F_V3F: types::GLenum = 0x2A2C;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C4UB_V3F: types::GLenum = 0x2A29;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_N3F_V3F: types::GLenum = 0x2A2B;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_V3F: types::GLenum = 0x2A27;
#[allow(dead_code, non_upper_case_globals)] pub const T4F_C4F_N3F_V4F: types::GLenum = 0x2A2D;
#[allow(dead_code, non_upper_case_globals)] pub const T4F_V4F: types::GLenum = 0x2A28;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BIT: types::GLenum = 0x00040000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER: types::GLenum = 0x1005;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPONENTS: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY: types::GLenum = 0x8078;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV: types::GLenum = 0x2300;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_COLOR: types::GLenum = 0x2201;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_MODE: types::GLenum = 0x2200;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_MODE: types::GLenum = 0x2500;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_Q: types::GLenum = 0x0C63;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_R: types::GLenum = 0x0C62;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_S: types::GLenum = 0x0C60;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_T: types::GLenum = 0x0C61;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTENSITY_SIZE: types::GLenum = 0x8061;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LUMINANCE_SIZE: types::GLenum = 0x8060;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MATRIX: types::GLenum = 0x0BA8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_PRIORITY: types::GLenum = 0x8066;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RESIDENT: types::GLenum = 0x8067;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STACK_DEPTH: types::GLenum = 0x0BA5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const V2F: types::GLenum = 0x2A20;
#[allow(dead_code, non_upper_case_globals)] pub const V3F: types::GLenum = 0x2A21;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_POINTER: types::GLenum = 0x808E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_SIZE: types::GLenum = 0x807A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_STRIDE: types::GLenum = 0x807C;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_TYPE: types::GLenum = 0x807B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const ZOOM_X: types::GLenum = 0x0D16;
#[allow(dead_code, non_upper_case_globals)] pub const ZOOM_Y: types::GLenum = 0x0D17;
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Accum(op: types::GLenum, value: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::Accum.f)(op, value) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn AlphaFunc(func: types::GLenum, ref_: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::AlphaFunc.f)(func, ref_) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn AreTexturesResident(n: types::GLsizei, textures: *const types::GLuint, residences: *mut types::GLboolean) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, *mut types::GLboolean) -> types::GLboolean>(storage::AreTexturesResident.f)(n, textures, residences) }
/// Fallbacks: ArrayElementEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ArrayElement(i: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::ArrayElement.f)(i) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Begin(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Begin.f)(mode) }
/// Fallbacks: BindTextureEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindTexture(target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Bitmap(width: types::GLsizei, height: types::GLsizei, xorig: types::GLfloat, yorig: types::GLfloat, xmove: types::GLfloat, ymove: types::GLfloat, bitmap: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLsizei, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, *const types::GLubyte) -> ()>(storage::Bitmap.f)(width, height, xorig, yorig, xmove, ymove, bitmap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CallList(list: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::CallList.f)(list) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CallLists(n: types::GLsizei, type_: types::GLenum, lists: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::CallLists.f)(n, type_, lists) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Clear(mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearAccum(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ClearAccum.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearDepth(depth: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(storage::ClearDepth.f)(depth) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearIndex(c: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::ClearIndex.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearStencil(s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::ClearStencil.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClipPlane(plane: types::GLenum, equation: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLdouble) -> ()>(storage::ClipPlane.f)(plane, equation) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3b(red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(storage::Color3b.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3bv(v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(storage::Color3bv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3d(red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Color3d.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Color3dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3f(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Color3f.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Color3fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3i(red: types::GLint, green: types::GLint, blue: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::Color3i.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Color3iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3s(red: types::GLshort, green: types::GLshort, blue: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Color3s.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Color3sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3ub(red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(storage::Color3ub.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3ubv(v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(storage::Color3ubv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3ui(red: types::GLuint, green: types::GLuint, blue: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Color3ui.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3uiv(v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLuint) -> ()>(storage::Color3uiv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3us(red: types::GLushort, green: types::GLushort, blue: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLushort, types::GLushort, types::GLushort) -> ()>(storage::Color3us.f)(red, green, blue) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color3usv(v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLushort) -> ()>(storage::Color3usv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4b(red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte, alpha: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(storage::Color4b.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4bv(v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(storage::Color4bv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4d(red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble, alpha: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Color4d.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Color4dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4f(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Color4f.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Color4fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4i(red: types::GLint, green: types::GLint, blue: types::GLint, alpha: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Color4i.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Color4iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4s(red: types::GLshort, green: types::GLshort, blue: types::GLshort, alpha: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Color4s.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Color4sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4ub(red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte, alpha: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(storage::Color4ub.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4ubv(v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(storage::Color4ubv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4ui(red: types::GLuint, green: types::GLuint, blue: types::GLuint, alpha: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Color4ui.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4uiv(v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLuint) -> ()>(storage::Color4uiv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4us(red: types::GLushort, green: types::GLushort, blue: types::GLushort, alpha: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLushort, types::GLushort, types::GLushort, types::GLushort) -> ()>(storage::Color4us.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4usv(v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLushort) -> ()>(storage::Color4usv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorMaterial(face: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::ColorMaterial.f)(face, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::ColorPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, type_: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum) -> ()>(storage::CopyPixels.f)(x, y, width, height, type_) }
/// Fallbacks: CopyTexImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
/// Fallbacks: CopyTexImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
/// Fallbacks: CopyTexSubImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
/// Fallbacks: CopyTexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CullFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::CullFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteLists(list: types::GLuint, range: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei) -> ()>(storage::DeleteLists.f)(list, range) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthFunc(func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthMask(flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(storage::DepthMask.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthRange(n: types::GLdouble, f: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::DepthRange.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Disable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Disable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DisableClientState(array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DisableClientState.f)(array) }
/// Fallbacks: DrawArraysEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(storage::DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawBuffer(buf: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DrawBuffer.f)(buf) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawPixels(width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawPixels.f)(width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EdgeFlag(flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(storage::EdgeFlag.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EdgeFlagPointer(stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::EdgeFlagPointer.f)(stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EdgeFlagv(flag: *const types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLboolean) -> ()>(storage::EdgeFlagv.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Enable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Enable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EnableClientState(array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::EnableClientState.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn End() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::End.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EndList() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::EndList.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord1d(u: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(storage::EvalCoord1d.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord1dv(u: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::EvalCoord1dv.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord1f(u: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::EvalCoord1f.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord1fv(u: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::EvalCoord1fv.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord2d(u: types::GLdouble, v: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::EvalCoord2d.f)(u, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord2dv(u: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::EvalCoord2dv.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord2f(u: types::GLfloat, v: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::EvalCoord2f.f)(u, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalCoord2fv(u: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::EvalCoord2fv.f)(u) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalMesh1(mode: types::GLenum, i1: types::GLint, i2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint) -> ()>(storage::EvalMesh1.f)(mode, i1, i2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalMesh2(mode: types::GLenum, i1: types::GLint, i2: types::GLint, j1: types::GLint, j2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::EvalMesh2.f)(mode, i1, i2, j1, j2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalPoint1(i: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::EvalPoint1.f)(i) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EvalPoint2(i: types::GLint, j: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::EvalPoint2.f)(i, j) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FeedbackBuffer(size: types::GLsizei, type_: types::GLenum, buffer: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLenum, *mut types::GLfloat) -> ()>(storage::FeedbackBuffer.f)(size, type_, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Finish() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Flush() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::Fogf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogfv(pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::Fogfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogi(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::Fogi.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogiv(pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(storage::Fogiv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FrontFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::FrontFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Frustum(left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Frustum.f)(left, right, bottom, top, zNear, zFar) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenLists(range: types::GLsizei) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei) -> types::GLuint>(storage::GenLists.f)(range) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(storage::GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetClipPlane(plane: types::GLenum, equation: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(storage::GetClipPlane.f)(plane, equation) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetDoublev(pname: types::GLenum, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(storage::GetDoublev.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetError() -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetFloatv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(storage::GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetLightfv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetLightfv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetLightiv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetLightiv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMapdv(target: types::GLenum, query: types::GLenum, v: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLdouble) -> ()>(storage::GetMapdv.f)(target, query, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMapfv(target: types::GLenum, query: types::GLenum, v: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetMapfv.f)(target, query, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMapiv(target: types::GLenum, query: types::GLenum, v: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetMapiv.f)(target, query, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMaterialfv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetMaterialfv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMaterialiv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetMaterialiv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPixelMapfv(map: types::GLenum, values: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetPixelMapfv.f)(map, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPixelMapuiv(map: types::GLenum, values: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLuint) -> ()>(storage::GetPixelMapuiv.f)(map, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPixelMapusv(map: types::GLenum, values: *mut types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLushort) -> ()>(storage::GetPixelMapusv.f)(map, values) }
/// Fallbacks: GetPointervEXT, GetPointervKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPointerv(pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetPointerv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPolygonStipple(mask: *mut types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*mut types::GLubyte) -> ()>(storage::GetPolygonStipple.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetString(name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(storage::GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexEnvfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexEnviv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexEnviv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexGendv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLdouble) -> ()>(storage::GetTexGendv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexGenfv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexGenfv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexGeniv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexGeniv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(storage::GetTexImage.f)(target, level, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Hint(target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IndexMask(mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::IndexMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IndexPointer(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::IndexPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexd(c: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(storage::Indexd.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexdv(c: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Indexdv.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexf(c: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::Indexf.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexfv(c: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Indexfv.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexi(c: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::Indexi.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexiv(c: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Indexiv.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexs(c: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort) -> ()>(storage::Indexs.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexsv(c: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Indexsv.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexub(c: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte) -> ()>(storage::Indexub.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Indexubv(c: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(storage::Indexubv.f)(c) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn InitNames() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::InitNames.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn InterleavedArrays(format: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::InterleavedArrays.f)(format, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsEnabled(cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::IsEnabled.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsList(list: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsList.f)(list) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsTexture(texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::LightModelf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelfv(pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::LightModelfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModeli(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::LightModeli.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModeliv(pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(storage::LightModeliv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightf(light: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::Lightf.f)(light, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightfv(light: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::Lightfv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lighti(light: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::Lighti.f)(light, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightiv(light: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::Lightiv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LineStipple(factor: types::GLint, pattern: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLushort) -> ()>(storage::LineStipple.f)(factor, pattern) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LineWidth(width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::LineWidth.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ListBase(base: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::ListBase.f)(base) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadIdentity() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::LoadIdentity.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadMatrixd(m: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::LoadMatrixd.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadMatrixf(m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::LoadMatrixf.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadName(name: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::LoadName.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LogicOp(opcode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::LogicOp.f)(opcode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Map1d(target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, stride: types::GLint, order: types::GLint, points: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLdouble, types::GLdouble, types::GLint, types::GLint, *const types::GLdouble) -> ()>(storage::Map1d.f)(target, u1, u2, stride, order, points) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Map1f(target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, stride: types::GLint, order: types::GLint, points: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLint, types::GLint, *const types::GLfloat) -> ()>(storage::Map1f.f)(target, u1, u2, stride, order, points) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Map2d(target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, ustride: types::GLint, uorder: types::GLint, v1: types::GLdouble, v2: types::GLdouble, vstride: types::GLint, vorder: types::GLint, points: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLdouble, types::GLdouble, types::GLint, types::GLint, types::GLdouble, types::GLdouble, types::GLint, types::GLint, *const types::GLdouble) -> ()>(storage::Map2d.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Map2f(target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, ustride: types::GLint, uorder: types::GLint, v1: types::GLfloat, v2: types::GLfloat, vstride: types::GLint, vorder: types::GLint, points: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLint, types::GLint, types::GLfloat, types::GLfloat, types::GLint, types::GLint, *const types::GLfloat) -> ()>(storage::Map2f.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapGrid1d(un: types::GLint, u1: types::GLdouble, u2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble) -> ()>(storage::MapGrid1d.f)(un, u1, u2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapGrid1f(un: types::GLint, u1: types::GLfloat, u2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::MapGrid1f.f)(un, u1, u2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapGrid2d(un: types::GLint, u1: types::GLdouble, u2: types::GLdouble, vn: types::GLint, v1: types::GLdouble, v2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLint, types::GLdouble, types::GLdouble) -> ()>(storage::MapGrid2d.f)(un, u1, u2, vn, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapGrid2f(un: types::GLint, u1: types::GLfloat, u2: types::GLfloat, vn: types::GLint, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::MapGrid2f.f)(un, u1, u2, vn, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialf(face: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::Materialf.f)(face, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialfv(face: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::Materialfv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materiali(face: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::Materiali.f)(face, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialiv(face: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::Materialiv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MatrixMode(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::MatrixMode.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultMatrixd(m: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::MultMatrixd.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultMatrixf(m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::MultMatrixf.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn NewList(list: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::NewList.f)(list, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3b(nx: types::GLbyte, ny: types::GLbyte, nz: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(storage::Normal3b.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3bv(v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(storage::Normal3bv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3d(nx: types::GLdouble, ny: types::GLdouble, nz: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Normal3d.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Normal3dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3f(nx: types::GLfloat, ny: types::GLfloat, nz: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Normal3f.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Normal3fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3i(nx: types::GLint, ny: types::GLint, nz: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::Normal3i.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Normal3iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3s(nx: types::GLshort, ny: types::GLshort, nz: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Normal3s.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Normal3sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn NormalPointer(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::NormalPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Ortho(left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Ortho.f)(left, right, bottom, top, zNear, zFar) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PassThrough(token: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::PassThrough.f)(token) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelMapfv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLfloat) -> ()>(storage::PixelMapfv.f)(map, mapsize, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelMapuiv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLuint) -> ()>(storage::PixelMapuiv.f)(map, mapsize, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelMapusv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLushort) -> ()>(storage::PixelMapusv.f)(map, mapsize, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelStoref(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::PixelStoref.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelStorei(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelTransferf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::PixelTransferf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelTransferi(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PixelTransferi.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelZoom(xfactor: types::GLfloat, yfactor: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::PixelZoom.f)(xfactor, yfactor) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointSize(size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::PointSize.f)(size) }
/// Fallbacks: PolygonModeNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonMode(face: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::PolygonMode.f)(face, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::PolygonOffset.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonStipple(mask: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(storage::PolygonStipple.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopAttrib() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopAttrib.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopClientAttrib() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopClientAttrib.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopMatrix() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopMatrix.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopName() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopName.f)() }
/// Fallbacks: PrioritizeTexturesEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PrioritizeTextures(n: types::GLsizei, textures: *const types::GLuint, priorities: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, *const types::GLfloat) -> ()>(storage::PrioritizeTextures.f)(n, textures, priorities) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushAttrib(mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::PushAttrib.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushClientAttrib(mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::PushClientAttrib.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushMatrix() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PushMatrix.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushName(name: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::PushName.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2d(x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::RasterPos2d.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::RasterPos2dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2f(x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::RasterPos2f.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::RasterPos2fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2i(x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::RasterPos2i.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::RasterPos2iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2s(x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(storage::RasterPos2s.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos2sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::RasterPos2sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::RasterPos3d.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::RasterPos3dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::RasterPos3f.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::RasterPos3fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3i(x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::RasterPos3i.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::RasterPos3iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3s(x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::RasterPos3s.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos3sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::RasterPos3sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::RasterPos4d.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::RasterPos4dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::RasterPos4f.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::RasterPos4fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4i(x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::RasterPos4i.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::RasterPos4iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4s(x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::RasterPos4s.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RasterPos4sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::RasterPos4sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadBuffer(src: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ReadBuffer.f)(src) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectd(x1: types::GLdouble, y1: types::GLdouble, x2: types::GLdouble, y2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Rectd.f)(x1, y1, x2, y2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectdv(v1: *const types::GLdouble, v2: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble, *const types::GLdouble) -> ()>(storage::Rectdv.f)(v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectf(x1: types::GLfloat, y1: types::GLfloat, x2: types::GLfloat, y2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Rectf.f)(x1, y1, x2, y2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectfv(v1: *const types::GLfloat, v2: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat, *const types::GLfloat) -> ()>(storage::Rectfv.f)(v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Recti(x1: types::GLint, y1: types::GLint, x2: types::GLint, y2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Recti.f)(x1, y1, x2, y2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectiv(v1: *const types::GLint, v2: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint, *const types::GLint) -> ()>(storage::Rectiv.f)(v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rects(x1: types::GLshort, y1: types::GLshort, x2: types::GLshort, y2: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Rects.f)(x1, y1, x2, y2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rectsv(v1: *const types::GLshort, v2: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort, *const types::GLshort) -> ()>(storage::Rectsv.f)(v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RenderMode(mode: types::GLenum) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLint>(storage::RenderMode.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rotated(angle: types::GLdouble, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Rotated.f)(angle, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rotatef(angle: types::GLfloat, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Rotatef.f)(angle, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scaled(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Scaled.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scalef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Scalef.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SelectBuffer(size: types::GLsizei, buffer: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::SelectBuffer.f)(size, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ShadeModel(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ShadeModel.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilMask(mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1d(s: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(storage::TexCoord1d.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::TexCoord1dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1f(s: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::TexCoord1f.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::TexCoord1fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1i(s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::TexCoord1i.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::TexCoord1iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1s(s: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort) -> ()>(storage::TexCoord1s.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord1sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::TexCoord1sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2d(s: types::GLdouble, t: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::TexCoord2d.f)(s, t) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::TexCoord2dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2f(s: types::GLfloat, t: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::TexCoord2f.f)(s, t) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::TexCoord2fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2i(s: types::GLint, t: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::TexCoord2i.f)(s, t) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::TexCoord2iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2s(s: types::GLshort, t: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(storage::TexCoord2s.f)(s, t) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord2sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::TexCoord2sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3d(s: types::GLdouble, t: types::GLdouble, r: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::TexCoord3d.f)(s, t, r) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::TexCoord3dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3f(s: types::GLfloat, t: types::GLfloat, r: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::TexCoord3f.f)(s, t, r) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::TexCoord3fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3i(s: types::GLint, t: types::GLint, r: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::TexCoord3i.f)(s, t, r) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::TexCoord3iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3s(s: types::GLshort, t: types::GLshort, r: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::TexCoord3s.f)(s, t, r) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord3sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::TexCoord3sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4d(s: types::GLdouble, t: types::GLdouble, r: types::GLdouble, q: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::TexCoord4d.f)(s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::TexCoord4dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4f(s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::TexCoord4f.f)(s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::TexCoord4fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4i(s: types::GLint, t: types::GLint, r: types::GLint, q: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::TexCoord4i.f)(s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::TexCoord4iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4s(s: types::GLshort, t: types::GLshort, r: types::GLshort, q: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::TexCoord4s.f)(s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoord4sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::TexCoord4sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoordPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::TexCoordPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexEnvf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvi(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexEnvi.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnviv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexEnviv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGend(coord: types::GLenum, pname: types::GLenum, param: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLdouble) -> ()>(storage::TexGend.f)(coord, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGendv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLdouble) -> ()>(storage::TexGendv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGenf(coord: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexGenf.f)(coord, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGenfv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexGenfv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGeni(coord: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexGeni.f)(coord, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexGeniv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexGeniv.f)(coord, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameteriv.f)(target, pname, params) }
/// Fallbacks: TexSubImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
/// Fallbacks: TexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Translated(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Translated.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Translatef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Translatef.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2d(x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::Vertex2d.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Vertex2dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2f(x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::Vertex2f.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Vertex2fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2i(x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::Vertex2i.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Vertex2iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2s(x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(storage::Vertex2s.f)(x, y) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex2sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Vertex2sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Vertex3d.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Vertex3dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Vertex3f.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Vertex3fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3i(x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::Vertex3i.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Vertex3iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3s(x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Vertex3s.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex3sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Vertex3sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Vertex4d.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4dv(v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(storage::Vertex4dv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Vertex4f.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4fv(v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::Vertex4fv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4i(x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Vertex4i.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4iv(v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::Vertex4iv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4s(x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::Vertex4s.f)(x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Vertex4sv(v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::Vertex4sv.f)(v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::VertexPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Viewport.f)(x, y, width, height) }

        #[allow(missing_copy_implementations)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr { f: missing_fn_panic as *const __gl_imports::raw::c_void, is_loaded: false }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }
        }
    
mod storage {
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]

    use super::__gl_imports::raw;
    use super::FnPtr;

    pub static mut Accum: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut AlphaFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut AreTexturesResident: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ArrayElement: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Begin: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Bitmap: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CallList: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CallLists: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Clear: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearAccum: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearDepth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearStencil: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClipPlane: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3b: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3bv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3ub: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3ubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3us: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color3usv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4b: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4bv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4ub: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4ubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4us: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4usv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorMaterial: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CullFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteLists: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Disable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DisableClientState: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EdgeFlag: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EdgeFlagPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EdgeFlagv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Enable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EnableClientState: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut End: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EndList: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalCoord2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalMesh1: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalMesh2: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalPoint1: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EvalPoint2: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FeedbackBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Finish: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Flush: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FrontFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Frustum: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenLists: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBooleanv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetClipPlane: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetDoublev: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetError: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFloatv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetIntegerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetLightfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetLightiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMapdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMapfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMapiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMaterialfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMaterialiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPixelMapfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPixelMapuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPixelMapusv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPolygonStipple: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetString: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexEnvfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexEnviv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexGendv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexGenfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexGeniv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Hint: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IndexMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IndexPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexd: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexs: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexsv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexub: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Indexubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut InitNames: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut InterleavedArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsEnabled: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsList: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModeli: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModeliv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lighti: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LineStipple: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LineWidth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ListBase: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadIdentity: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadMatrixd: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadMatrixf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LogicOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Map1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Map1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Map2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Map2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapGrid1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapGrid1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapGrid2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapGrid2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materiali: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MatrixMode: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultMatrixd: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultMatrixf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut NewList: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3b: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3bv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut NormalPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Ortho: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PassThrough: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelMapfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelMapuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelMapusv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelStoref: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelStorei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelTransferf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelTransferi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelZoom: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointSize: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonMode: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonOffset: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonStipple: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopClientAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopMatrix: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PrioritizeTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushClientAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushMatrix: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos2sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RasterPos4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectd: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Recti: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rects: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rectsv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RenderMode: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rotated: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rotatef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scaled: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scalef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scissor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SelectBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ShadeModel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord1sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord2sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoord4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoordPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnviv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGend: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGendv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGenf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGenfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGeni: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexGeniv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Translated: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Translatef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex2sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Vertex4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Viewport: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
}

            #[allow(non_snake_case)]
            pub mod Accum {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Accum.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Accum = FnPtr::new(metaloadfn(&mut loadfn, "glAccum", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod AlphaFunc {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::AlphaFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::AlphaFunc = FnPtr::new(metaloadfn(&mut loadfn, "glAlphaFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod AreTexturesResident {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::AreTexturesResident.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::AreTexturesResident = FnPtr::new(metaloadfn(&mut loadfn, "glAreTexturesResident", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ArrayElement {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ArrayElement.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ArrayElement = FnPtr::new(metaloadfn(&mut loadfn, "glArrayElement", &["glArrayElementEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Begin {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Begin.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Begin = FnPtr::new(metaloadfn(&mut loadfn, "glBegin", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindTexture {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BindTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindTexture", &["glBindTextureEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Bitmap {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Bitmap.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Bitmap = FnPtr::new(metaloadfn(&mut loadfn, "glBitmap", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFunc {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFunc = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CallList {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CallList.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CallList = FnPtr::new(metaloadfn(&mut loadfn, "glCallList", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CallLists {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CallLists.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CallLists = FnPtr::new(metaloadfn(&mut loadfn, "glCallLists", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Clear {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Clear.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Clear = FnPtr::new(metaloadfn(&mut loadfn, "glClear", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearAccum {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearAccum.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearAccum = FnPtr::new(metaloadfn(&mut loadfn, "glClearAccum", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearColor {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearColor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearColor = FnPtr::new(metaloadfn(&mut loadfn, "glClearColor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearDepth {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearDepth.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearDepth = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepth", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearIndex {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearIndex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearIndex = FnPtr::new(metaloadfn(&mut loadfn, "glClearIndex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearStencil {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearStencil.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearStencil = FnPtr::new(metaloadfn(&mut loadfn, "glClearStencil", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClipPlane {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClipPlane.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClipPlane = FnPtr::new(metaloadfn(&mut loadfn, "glClipPlane", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3b {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3b.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3b = FnPtr::new(metaloadfn(&mut loadfn, "glColor3b", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3bv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3bv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3bv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3bv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3d = FnPtr::new(metaloadfn(&mut loadfn, "glColor3d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3dv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3f = FnPtr::new(metaloadfn(&mut loadfn, "glColor3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3fv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3i = FnPtr::new(metaloadfn(&mut loadfn, "glColor3i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3iv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3s = FnPtr::new(metaloadfn(&mut loadfn, "glColor3s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3sv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3ub {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3ub.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3ub = FnPtr::new(metaloadfn(&mut loadfn, "glColor3ub", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3ubv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3ubv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3ubv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3ubv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3ui {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3ui = FnPtr::new(metaloadfn(&mut loadfn, "glColor3ui", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3uiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3uiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3us {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3us.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3us = FnPtr::new(metaloadfn(&mut loadfn, "glColor3us", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color3usv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color3usv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color3usv = FnPtr::new(metaloadfn(&mut loadfn, "glColor3usv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4b {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4b.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4b = FnPtr::new(metaloadfn(&mut loadfn, "glColor4b", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4bv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4bv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4bv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4bv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4d = FnPtr::new(metaloadfn(&mut loadfn, "glColor4d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4dv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4f = FnPtr::new(metaloadfn(&mut loadfn, "glColor4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4fv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4i = FnPtr::new(metaloadfn(&mut loadfn, "glColor4i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4iv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4s = FnPtr::new(metaloadfn(&mut loadfn, "glColor4s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4sv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4ub {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4ub.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4ub = FnPtr::new(metaloadfn(&mut loadfn, "glColor4ub", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4ubv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4ubv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4ubv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4ubv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4ui {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4ui = FnPtr::new(metaloadfn(&mut loadfn, "glColor4ui", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4uiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4uiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4us {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4us.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4us = FnPtr::new(metaloadfn(&mut loadfn, "glColor4us", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4usv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4usv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4usv = FnPtr::new(metaloadfn(&mut loadfn, "glColor4usv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorMask {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorMask = FnPtr::new(metaloadfn(&mut loadfn, "glColorMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorMaterial {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorMaterial.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorMaterial = FnPtr::new(metaloadfn(&mut loadfn, "glColorMaterial", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorPointer = FnPtr::new(metaloadfn(&mut loadfn, "glColorPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyPixels {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyPixels = FnPtr::new(metaloadfn(&mut loadfn, "glCopyPixels", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexImage1D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexImage1D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage1D", &["glCopyTexImage1DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexImage2D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage2D", &["glCopyTexImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexSubImage1D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexSubImage1D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage1D", &["glCopyTexSubImage1DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexSubImage2D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CullFace {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CullFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CullFace = FnPtr::new(metaloadfn(&mut loadfn, "glCullFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteLists {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteLists.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteLists = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteLists", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteTextures {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteTextures = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthFunc {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthFunc = FnPtr::new(metaloadfn(&mut loadfn, "glDepthFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthMask {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthMask = FnPtr::new(metaloadfn(&mut loadfn, "glDepthMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthRange {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthRange.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthRange = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRange", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Disable {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Disable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Disable = FnPtr::new(metaloadfn(&mut loadfn, "glDisable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DisableClientState {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DisableClientState.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DisableClientState = FnPtr::new(metaloadfn(&mut loadfn, "glDisableClientState", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawArrays {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArrays", &["glDrawArraysEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawBuffer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glDrawBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElements {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElements.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElements", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawPixels {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawPixels = FnPtr::new(metaloadfn(&mut loadfn, "glDrawPixels", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EdgeFlag {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EdgeFlag.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EdgeFlag = FnPtr::new(metaloadfn(&mut loadfn, "glEdgeFlag", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EdgeFlagPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EdgeFlagPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EdgeFlagPointer = FnPtr::new(metaloadfn(&mut loadfn, "glEdgeFlagPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EdgeFlagv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EdgeFlagv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EdgeFlagv = FnPtr::new(metaloadfn(&mut loadfn, "glEdgeFlagv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Enable {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Enable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Enable = FnPtr::new(metaloadfn(&mut loadfn, "glEnable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EnableClientState {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EnableClientState.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EnableClientState = FnPtr::new(metaloadfn(&mut loadfn, "glEnableClientState", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod End {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::End.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::End = FnPtr::new(metaloadfn(&mut loadfn, "glEnd", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EndList {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EndList.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EndList = FnPtr::new(metaloadfn(&mut loadfn, "glEndList", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord1d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord1d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord1d = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord1d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord1dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord1dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord1dv = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord1dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord1f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord1f = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord1f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord1fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord1fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord1fv = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord1fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord2d = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord2dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord2dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord2dv = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord2dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord2f = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalCoord2fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalCoord2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalCoord2fv = FnPtr::new(metaloadfn(&mut loadfn, "glEvalCoord2fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalMesh1 {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalMesh1.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalMesh1 = FnPtr::new(metaloadfn(&mut loadfn, "glEvalMesh1", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalMesh2 {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalMesh2.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalMesh2 = FnPtr::new(metaloadfn(&mut loadfn, "glEvalMesh2", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalPoint1 {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalPoint1.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalPoint1 = FnPtr::new(metaloadfn(&mut loadfn, "glEvalPoint1", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EvalPoint2 {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EvalPoint2.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EvalPoint2 = FnPtr::new(metaloadfn(&mut loadfn, "glEvalPoint2", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FeedbackBuffer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FeedbackBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::FeedbackBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glFeedbackBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Finish {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Finish.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Finish = FnPtr::new(metaloadfn(&mut loadfn, "glFinish", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Flush {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Flush.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Flush = FnPtr::new(metaloadfn(&mut loadfn, "glFlush", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogf = FnPtr::new(metaloadfn(&mut loadfn, "glFogf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogfv = FnPtr::new(metaloadfn(&mut loadfn, "glFogfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogi {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogi = FnPtr::new(metaloadfn(&mut loadfn, "glFogi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogiv = FnPtr::new(metaloadfn(&mut loadfn, "glFogiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FrontFace {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FrontFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::FrontFace = FnPtr::new(metaloadfn(&mut loadfn, "glFrontFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Frustum {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Frustum.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Frustum = FnPtr::new(metaloadfn(&mut loadfn, "glFrustum", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenLists {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenLists.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenLists = FnPtr::new(metaloadfn(&mut loadfn, "glGenLists", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenTextures {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenTextures = FnPtr::new(metaloadfn(&mut loadfn, "glGenTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBooleanv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBooleanv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBooleanv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleanv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetClipPlane {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetClipPlane.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetClipPlane = FnPtr::new(metaloadfn(&mut loadfn, "glGetClipPlane", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetDoublev {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetDoublev.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetDoublev = FnPtr::new(metaloadfn(&mut loadfn, "glGetDoublev", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetError {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetError.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetError = FnPtr::new(metaloadfn(&mut loadfn, "glGetError", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFloatv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFloatv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFloatv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFloatv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetIntegerv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetIntegerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetIntegerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegerv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetLightfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetLightfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetLightfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetLightfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetLightiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetLightiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetLightiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetLightiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMapdv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMapdv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMapdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMapdv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMapfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMapfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMapfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMapfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMapiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMapiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMapiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMapiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMaterialfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMaterialfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMaterialfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMaterialfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMaterialiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMaterialiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMaterialiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMaterialiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPixelMapfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPixelMapfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPixelMapfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPixelMapfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPixelMapuiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPixelMapuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPixelMapuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPixelMapuiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPixelMapusv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPixelMapusv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPixelMapusv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPixelMapusv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPointerv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPointerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPolygonStipple {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPolygonStipple.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPolygonStipple = FnPtr::new(metaloadfn(&mut loadfn, "glGetPolygonStipple", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetString {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetString.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetString = FnPtr::new(metaloadfn(&mut loadfn, "glGetString", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexEnvfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexEnvfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexEnvfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexEnvfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexEnviv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexEnviv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexEnviv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexEnviv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexGendv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexGendv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexGendv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexGendv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexGenfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexGenfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexGenfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexGenfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexGeniv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexGeniv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexGeniv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexGeniv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexImage {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexImage.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexImage", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexLevelParameterfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexLevelParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexLevelParameteriv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexLevelParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameteriv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Hint {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Hint.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Hint = FnPtr::new(metaloadfn(&mut loadfn, "glHint", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IndexMask {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IndexMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IndexMask = FnPtr::new(metaloadfn(&mut loadfn, "glIndexMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IndexPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IndexPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IndexPointer = FnPtr::new(metaloadfn(&mut loadfn, "glIndexPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexd {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexd.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexd = FnPtr::new(metaloadfn(&mut loadfn, "glIndexd", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexdv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexdv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexdv = FnPtr::new(metaloadfn(&mut loadfn, "glIndexdv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexf = FnPtr::new(metaloadfn(&mut loadfn, "glIndexf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexfv = FnPtr::new(metaloadfn(&mut loadfn, "glIndexfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexi {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexi = FnPtr::new(metaloadfn(&mut loadfn, "glIndexi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexiv = FnPtr::new(metaloadfn(&mut loadfn, "glIndexiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexs {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexs.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexs = FnPtr::new(metaloadfn(&mut loadfn, "glIndexs", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexsv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexsv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexsv = FnPtr::new(metaloadfn(&mut loadfn, "glIndexsv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexub {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexub.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexub = FnPtr::new(metaloadfn(&mut loadfn, "glIndexub", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Indexubv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Indexubv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Indexubv = FnPtr::new(metaloadfn(&mut loadfn, "glIndexubv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod InitNames {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::InitNames.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::InitNames = FnPtr::new(metaloadfn(&mut loadfn, "glInitNames", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod InterleavedArrays {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::InterleavedArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::InterleavedArrays = FnPtr::new(metaloadfn(&mut loadfn, "glInterleavedArrays", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsEnabled {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsEnabled.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsEnabled = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabled", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsList {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsList.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsList = FnPtr::new(metaloadfn(&mut loadfn, "glIsList", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsTexture {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsTexture = FnPtr::new(metaloadfn(&mut loadfn, "glIsTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelf = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelfv = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModeli {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModeli.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModeli = FnPtr::new(metaloadfn(&mut loadfn, "glLightModeli", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModeliv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModeliv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModeliv = FnPtr::new(metaloadfn(&mut loadfn, "glLightModeliv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightf = FnPtr::new(metaloadfn(&mut loadfn, "glLightf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightfv = FnPtr::new(metaloadfn(&mut loadfn, "glLightfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lighti {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lighti.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lighti = FnPtr::new(metaloadfn(&mut loadfn, "glLighti", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightiv = FnPtr::new(metaloadfn(&mut loadfn, "glLightiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LineStipple {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LineStipple.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LineStipple = FnPtr::new(metaloadfn(&mut loadfn, "glLineStipple", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LineWidth {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LineWidth.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LineWidth = FnPtr::new(metaloadfn(&mut loadfn, "glLineWidth", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ListBase {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ListBase.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ListBase = FnPtr::new(metaloadfn(&mut loadfn, "glListBase", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadIdentity {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadIdentity.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadIdentity = FnPtr::new(metaloadfn(&mut loadfn, "glLoadIdentity", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadMatrixd {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadMatrixd.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadMatrixd = FnPtr::new(metaloadfn(&mut loadfn, "glLoadMatrixd", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadMatrixf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadMatrixf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadMatrixf = FnPtr::new(metaloadfn(&mut loadfn, "glLoadMatrixf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadName {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadName.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadName = FnPtr::new(metaloadfn(&mut loadfn, "glLoadName", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LogicOp {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LogicOp.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LogicOp = FnPtr::new(metaloadfn(&mut loadfn, "glLogicOp", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Map1d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Map1d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Map1d = FnPtr::new(metaloadfn(&mut loadfn, "glMap1d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Map1f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Map1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Map1f = FnPtr::new(metaloadfn(&mut loadfn, "glMap1f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Map2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Map2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Map2d = FnPtr::new(metaloadfn(&mut loadfn, "glMap2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Map2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Map2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Map2f = FnPtr::new(metaloadfn(&mut loadfn, "glMap2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapGrid1d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapGrid1d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MapGrid1d = FnPtr::new(metaloadfn(&mut loadfn, "glMapGrid1d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapGrid1f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapGrid1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MapGrid1f = FnPtr::new(metaloadfn(&mut loadfn, "glMapGrid1f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapGrid2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapGrid2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MapGrid2d = FnPtr::new(metaloadfn(&mut loadfn, "glMapGrid2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapGrid2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapGrid2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MapGrid2f = FnPtr::new(metaloadfn(&mut loadfn, "glMapGrid2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialf = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialfv = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materiali {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materiali.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materiali = FnPtr::new(metaloadfn(&mut loadfn, "glMateriali", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialiv = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MatrixMode {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MatrixMode.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MatrixMode = FnPtr::new(metaloadfn(&mut loadfn, "glMatrixMode", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultMatrixd {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultMatrixd.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultMatrixd = FnPtr::new(metaloadfn(&mut loadfn, "glMultMatrixd", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultMatrixf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultMatrixf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultMatrixf = FnPtr::new(metaloadfn(&mut loadfn, "glMultMatrixf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod NewList {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::NewList.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::NewList = FnPtr::new(metaloadfn(&mut loadfn, "glNewList", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3b {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3b.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3b = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3b", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3bv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3bv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3bv = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3bv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3d = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3dv = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3f = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3fv = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3i = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3iv = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3s = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3sv = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod NormalPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::NormalPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::NormalPointer = FnPtr::new(metaloadfn(&mut loadfn, "glNormalPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Ortho {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Ortho.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Ortho = FnPtr::new(metaloadfn(&mut loadfn, "glOrtho", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PassThrough {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PassThrough.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PassThrough = FnPtr::new(metaloadfn(&mut loadfn, "glPassThrough", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelMapfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelMapfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelMapfv = FnPtr::new(metaloadfn(&mut loadfn, "glPixelMapfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelMapuiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelMapuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelMapuiv = FnPtr::new(metaloadfn(&mut loadfn, "glPixelMapuiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelMapusv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelMapusv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelMapusv = FnPtr::new(metaloadfn(&mut loadfn, "glPixelMapusv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelStoref {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelStoref.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelStoref = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStoref", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelStorei {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelStorei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelStorei = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStorei", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelTransferf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelTransferf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelTransferf = FnPtr::new(metaloadfn(&mut loadfn, "glPixelTransferf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelTransferi {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelTransferi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelTransferi = FnPtr::new(metaloadfn(&mut loadfn, "glPixelTransferi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelZoom {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelZoom.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelZoom = FnPtr::new(metaloadfn(&mut loadfn, "glPixelZoom", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointSize {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointSize.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointSize = FnPtr::new(metaloadfn(&mut loadfn, "glPointSize", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonMode {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonMode.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonMode = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonMode", &["glPolygonModeNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonOffset {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonOffset.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonOffset = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonOffset", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonStipple {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonStipple.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonStipple = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonStipple", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopAttrib {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopAttrib.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PopAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glPopAttrib", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopClientAttrib {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopClientAttrib.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PopClientAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glPopClientAttrib", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopMatrix {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopMatrix.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PopMatrix = FnPtr::new(metaloadfn(&mut loadfn, "glPopMatrix", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopName {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopName.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PopName = FnPtr::new(metaloadfn(&mut loadfn, "glPopName", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PrioritizeTextures {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PrioritizeTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PrioritizeTextures = FnPtr::new(metaloadfn(&mut loadfn, "glPrioritizeTextures", &["glPrioritizeTexturesEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushAttrib {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushAttrib.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PushAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glPushAttrib", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushClientAttrib {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushClientAttrib.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PushClientAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glPushClientAttrib", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushMatrix {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushMatrix.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PushMatrix = FnPtr::new(metaloadfn(&mut loadfn, "glPushMatrix", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushName {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushName.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PushName = FnPtr::new(metaloadfn(&mut loadfn, "glPushName", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2d = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2dv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2f = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2fv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2i = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2iv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2s = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos2sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos2sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos2sv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos2sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3d = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3dv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3f = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3fv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3i = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3iv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3s = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos3sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos3sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos3sv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos3sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4d = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4dv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4f = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4fv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4i = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4iv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4s = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RasterPos4sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RasterPos4sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RasterPos4sv = FnPtr::new(metaloadfn(&mut loadfn, "glRasterPos4sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadBuffer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glReadBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadPixels {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadPixels", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectd {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectd.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectd = FnPtr::new(metaloadfn(&mut loadfn, "glRectd", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectdv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectdv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectdv = FnPtr::new(metaloadfn(&mut loadfn, "glRectdv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectf = FnPtr::new(metaloadfn(&mut loadfn, "glRectf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectfv = FnPtr::new(metaloadfn(&mut loadfn, "glRectfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Recti {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Recti.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Recti = FnPtr::new(metaloadfn(&mut loadfn, "glRecti", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectiv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectiv = FnPtr::new(metaloadfn(&mut loadfn, "glRectiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rects {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rects.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rects = FnPtr::new(metaloadfn(&mut loadfn, "glRects", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rectsv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rectsv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rectsv = FnPtr::new(metaloadfn(&mut loadfn, "glRectsv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RenderMode {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RenderMode.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RenderMode = FnPtr::new(metaloadfn(&mut loadfn, "glRenderMode", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rotated {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rotated.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rotated = FnPtr::new(metaloadfn(&mut loadfn, "glRotated", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rotatef {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rotatef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rotatef = FnPtr::new(metaloadfn(&mut loadfn, "glRotatef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scaled {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scaled.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scaled = FnPtr::new(metaloadfn(&mut loadfn, "glScaled", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scalef {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scalef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scalef = FnPtr::new(metaloadfn(&mut loadfn, "glScalef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scissor {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scissor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scissor = FnPtr::new(metaloadfn(&mut loadfn, "glScissor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SelectBuffer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SelectBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::SelectBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glSelectBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ShadeModel {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ShadeModel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ShadeModel = FnPtr::new(metaloadfn(&mut loadfn, "glShadeModel", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilFunc {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilFunc = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilMask {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilMask = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilOp {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilOp.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilOp = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOp", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1d = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1dv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1f = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1fv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1i = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1iv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1s = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord1sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord1sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord1sv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord1sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2d = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2dv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2f = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2fv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2i = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2iv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2s = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord2sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord2sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord2sv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord2sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3d = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3dv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3f = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3fv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3i = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3iv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3s = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord3sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord3sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord3sv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord3sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4d = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4dv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4f = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4fv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4i = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4iv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4s = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoord4sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoord4sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoord4sv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoord4sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoordPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoordPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoordPointer = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvf = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvi {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvi = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnviv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnviv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnviv = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnviv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGend {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGend.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGend = FnPtr::new(metaloadfn(&mut loadfn, "glTexGend", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGendv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGendv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGendv = FnPtr::new(metaloadfn(&mut loadfn, "glTexGendv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGenf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGenf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGenf = FnPtr::new(metaloadfn(&mut loadfn, "glTexGenf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGenfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGenfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGenfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexGenfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGeni {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGeni.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGeni = FnPtr::new(metaloadfn(&mut loadfn, "glTexGeni", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexGeniv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexGeniv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexGeniv = FnPtr::new(metaloadfn(&mut loadfn, "glTexGeniv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexImage1D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexImage1D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage1D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexImage2D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterf {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterfv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteri {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteri", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteriv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexSubImage1D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexSubImage1D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage1D", &["glTexSubImage1DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexSubImage2D {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage2D", &["glTexSubImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Translated {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Translated.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Translated = FnPtr::new(metaloadfn(&mut loadfn, "glTranslated", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Translatef {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Translatef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Translatef = FnPtr::new(metaloadfn(&mut loadfn, "glTranslatef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2d = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2f = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2i = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2s = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex2sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex2sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex2sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex2sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3d = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3f = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3i = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3s = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex3sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex3sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex3sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex3sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4d {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4d.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4d = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4d", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4dv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4dv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4dv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4f {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4f = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4fv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4fv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4i {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4i = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4i", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4iv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4iv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4s {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4s.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4s = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4s", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Vertex4sv {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Vertex4sv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Vertex4sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertex4sv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexPointer {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Viewport {
                use super::{metaloadfn, storage};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Viewport.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Viewport = FnPtr::new(metaloadfn(&mut loadfn, "glViewport", &[]))
                    }
                }
            }
        
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gl function was not loaded")
        }
        

        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {
            #[inline(never)]
            fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void) {
    
Accum::load_with(&mut *loadfn);
AlphaFunc::load_with(&mut *loadfn);
AreTexturesResident::load_with(&mut *loadfn);
ArrayElement::load_with(&mut *loadfn);
Begin::load_with(&mut *loadfn);
BindTexture::load_with(&mut *loadfn);
Bitmap::load_with(&mut *loadfn);
BlendFunc::load_with(&mut *loadfn);
CallList::load_with(&mut *loadfn);
CallLists::load_with(&mut *loadfn);
Clear::load_with(&mut *loadfn);
ClearAccum::load_with(&mut *loadfn);
ClearColor::load_with(&mut *loadfn);
ClearDepth::load_with(&mut *loadfn);
ClearIndex::load_with(&mut *loadfn);
ClearStencil::load_with(&mut *loadfn);
ClipPlane::load_with(&mut *loadfn);
Color3b::load_with(&mut *loadfn);
Color3bv::load_with(&mut *loadfn);
Color3d::load_with(&mut *loadfn);
Color3dv::load_with(&mut *loadfn);
Color3f::load_with(&mut *loadfn);
Color3fv::load_with(&mut *loadfn);
Color3i::load_with(&mut *loadfn);
Color3iv::load_with(&mut *loadfn);
Color3s::load_with(&mut *loadfn);
Color3sv::load_with(&mut *loadfn);
Color3ub::load_with(&mut *loadfn);
Color3ubv::load_with(&mut *loadfn);
Color3ui::load_with(&mut *loadfn);
Color3uiv::load_with(&mut *loadfn);
Color3us::load_with(&mut *loadfn);
Color3usv::load_with(&mut *loadfn);
Color4b::load_with(&mut *loadfn);
Color4bv::load_with(&mut *loadfn);
Color4d::load_with(&mut *loadfn);
Color4dv::load_with(&mut *loadfn);
Color4f::load_with(&mut *loadfn);
Color4fv::load_with(&mut *loadfn);
Color4i::load_with(&mut *loadfn);
Color4iv::load_with(&mut *loadfn);
Color4s::load_with(&mut *loadfn);
Color4sv::load_with(&mut *loadfn);
Color4ub::load_with(&mut *loadfn);
Color4ubv::load_with(&mut *loadfn);
Color4ui::load_with(&mut *loadfn);
Color4uiv::load_with(&mut *loadfn);
Color4us::load_with(&mut *loadfn);
Color4usv::load_with(&mut *loadfn);
ColorMask::load_with(&mut *loadfn);
ColorMaterial::load_with(&mut *loadfn);
ColorPointer::load_with(&mut *loadfn);
CopyPixels::load_with(&mut *loadfn);
CopyTexImage1D::load_with(&mut *loadfn);
CopyTexImage2D::load_with(&mut *loadfn);
CopyTexSubImage1D::load_with(&mut *loadfn);
CopyTexSubImage2D::load_with(&mut *loadfn);
CullFace::load_with(&mut *loadfn);
DeleteLists::load_with(&mut *loadfn);
DeleteTextures::load_with(&mut *loadfn);
DepthFunc::load_with(&mut *loadfn);
DepthMask::load_with(&mut *loadfn);
DepthRange::load_with(&mut *loadfn);
Disable::load_with(&mut *loadfn);
DisableClientState::load_with(&mut *loadfn);
DrawArrays::load_with(&mut *loadfn);
DrawBuffer::load_with(&mut *loadfn);
DrawElements::load_with(&mut *loadfn);
DrawPixels::load_with(&mut *loadfn);
EdgeFlag::load_with(&mut *loadfn);
EdgeFlagPointer::load_with(&mut *loadfn);
EdgeFlagv::load_with(&mut *loadfn);
Enable::load_with(&mut *loadfn);
EnableClientState::load_with(&mut *loadfn);
End::load_with(&mut *loadfn);
EndList::load_with(&mut *loadfn);
EvalCoord1d::load_with(&mut *loadfn);
EvalCoord1dv::load_with(&mut *loadfn);
EvalCoord1f::load_with(&mut *loadfn);
EvalCoord1fv::load_with(&mut *loadfn);
EvalCoord2d::load_with(&mut *loadfn);
EvalCoord2dv::load_with(&mut *loadfn);
EvalCoord2f::load_with(&mut *loadfn);
EvalCoord2fv::load_with(&mut *loadfn);
EvalMesh1::load_with(&mut *loadfn);
EvalMesh2::load_with(&mut *loadfn);
EvalPoint1::load_with(&mut *loadfn);
EvalPoint2::load_with(&mut *loadfn);
FeedbackBuffer::load_with(&mut *loadfn);
Finish::load_with(&mut *loadfn);
Flush::load_with(&mut *loadfn);
Fogf::load_with(&mut *loadfn);
Fogfv::load_with(&mut *loadfn);
Fogi::load_with(&mut *loadfn);
Fogiv::load_with(&mut *loadfn);
FrontFace::load_with(&mut *loadfn);
Frustum::load_with(&mut *loadfn);
GenLists::load_with(&mut *loadfn);
GenTextures::load_with(&mut *loadfn);
GetBooleanv::load_with(&mut *loadfn);
GetClipPlane::load_with(&mut *loadfn);
GetDoublev::load_with(&mut *loadfn);
GetError::load_with(&mut *loadfn);
GetFloatv::load_with(&mut *loadfn);
GetIntegerv::load_with(&mut *loadfn);
GetLightfv::load_with(&mut *loadfn);
GetLightiv::load_with(&mut *loadfn);
GetMapdv::load_with(&mut *loadfn);
GetMapfv::load_with(&mut *loadfn);
GetMapiv::load_with(&mut *loadfn);
GetMaterialfv::load_with(&mut *loadfn);
GetMaterialiv::load_with(&mut *loadfn);
GetPixelMapfv::load_with(&mut *loadfn);
GetPixelMapuiv::load_with(&mut *loadfn);
GetPixelMapusv::load_with(&mut *loadfn);
GetPointerv::load_with(&mut *loadfn);
GetPolygonStipple::load_with(&mut *loadfn);
GetString::load_with(&mut *loadfn);
GetTexEnvfv::load_with(&mut *loadfn);
GetTexEnviv::load_with(&mut *loadfn);
GetTexGendv::load_with(&mut *loadfn);
GetTexGenfv::load_with(&mut *loadfn);
GetTexGeniv::load_with(&mut *loadfn);
GetTexImage::load_with(&mut *loadfn);
GetTexLevelParameterfv::load_with(&mut *loadfn);
GetTexLevelParameteriv::load_with(&mut *loadfn);
GetTexParameterfv::load_with(&mut *loadfn);
GetTexParameteriv::load_with(&mut *loadfn);
Hint::load_with(&mut *loadfn);
IndexMask::load_with(&mut *loadfn);
IndexPointer::load_with(&mut *loadfn);
Indexd::load_with(&mut *loadfn);
Indexdv::load_with(&mut *loadfn);
Indexf::load_with(&mut *loadfn);
Indexfv::load_with(&mut *loadfn);
Indexi::load_with(&mut *loadfn);
Indexiv::load_with(&mut *loadfn);
Indexs::load_with(&mut *loadfn);
Indexsv::load_with(&mut *loadfn);
Indexub::load_with(&mut *loadfn);
Indexubv::load_with(&mut *loadfn);
InitNames::load_with(&mut *loadfn);
InterleavedArrays::load_with(&mut *loadfn);
IsEnabled::load_with(&mut *loadfn);
IsList::load_with(&mut *loadfn);
IsTexture::load_with(&mut *loadfn);
LightModelf::load_with(&mut *loadfn);
LightModelfv::load_with(&mut *loadfn);
LightModeli::load_with(&mut *loadfn);
LightModeliv::load_with(&mut *loadfn);
Lightf::load_with(&mut *loadfn);
Lightfv::load_with(&mut *loadfn);
Lighti::load_with(&mut *loadfn);
Lightiv::load_with(&mut *loadfn);
LineStipple::load_with(&mut *loadfn);
LineWidth::load_with(&mut *loadfn);
ListBase::load_with(&mut *loadfn);
LoadIdentity::load_with(&mut *loadfn);
LoadMatrixd::load_with(&mut *loadfn);
LoadMatrixf::load_with(&mut *loadfn);
LoadName::load_with(&mut *loadfn);
LogicOp::load_with(&mut *loadfn);
Map1d::load_with(&mut *loadfn);
Map1f::load_with(&mut *loadfn);
Map2d::load_with(&mut *loadfn);
Map2f::load_with(&mut *loadfn);
MapGrid1d::load_with(&mut *loadfn);
MapGrid1f::load_with(&mut *loadfn);
MapGrid2d::load_with(&mut *loadfn);
MapGrid2f::load_with(&mut *loadfn);
Materialf::load_with(&mut *loadfn);
Materialfv::load_with(&mut *loadfn);
Materiali::load_with(&mut *loadfn);
Materialiv::load_with(&mut *loadfn);
MatrixMode::load_with(&mut *loadfn);
MultMatrixd::load_with(&mut *loadfn);
MultMatrixf::load_with(&mut *loadfn);
NewList::load_with(&mut *loadfn);
Normal3b::load_with(&mut *loadfn);
Normal3bv::load_with(&mut *loadfn);
Normal3d::load_with(&mut *loadfn);
Normal3dv::load_with(&mut *loadfn);
Normal3f::load_with(&mut *loadfn);
Normal3fv::load_with(&mut *loadfn);
Normal3i::load_with(&mut *loadfn);
Normal3iv::load_with(&mut *loadfn);
Normal3s::load_with(&mut *loadfn);
Normal3sv::load_with(&mut *loadfn);
NormalPointer::load_with(&mut *loadfn);
Ortho::load_with(&mut *loadfn);
PassThrough::load_with(&mut *loadfn);
PixelMapfv::load_with(&mut *loadfn);
PixelMapuiv::load_with(&mut *loadfn);
PixelMapusv::load_with(&mut *loadfn);
PixelStoref::load_with(&mut *loadfn);
PixelStorei::load_with(&mut *loadfn);
PixelTransferf::load_with(&mut *loadfn);
PixelTransferi::load_with(&mut *loadfn);
PixelZoom::load_with(&mut *loadfn);
PointSize::load_with(&mut *loadfn);
PolygonMode::load_with(&mut *loadfn);
PolygonOffset::load_with(&mut *loadfn);
PolygonStipple::load_with(&mut *loadfn);
PopAttrib::load_with(&mut *loadfn);
PopClientAttrib::load_with(&mut *loadfn);
PopMatrix::load_with(&mut *loadfn);
PopName::load_with(&mut *loadfn);
PrioritizeTextures::load_with(&mut *loadfn);
PushAttrib::load_with(&mut *loadfn);
PushClientAttrib::load_with(&mut *loadfn);
PushMatrix::load_with(&mut *loadfn);
PushName::load_with(&mut *loadfn);
RasterPos2d::load_with(&mut *loadfn);
RasterPos2dv::load_with(&mut *loadfn);
RasterPos2f::load_with(&mut *loadfn);
RasterPos2fv::load_with(&mut *loadfn);
RasterPos2i::load_with(&mut *loadfn);
RasterPos2iv::load_with(&mut *loadfn);
RasterPos2s::load_with(&mut *loadfn);
RasterPos2sv::load_with(&mut *loadfn);
RasterPos3d::load_with(&mut *loadfn);
RasterPos3dv::load_with(&mut *loadfn);
RasterPos3f::load_with(&mut *loadfn);
RasterPos3fv::load_with(&mut *loadfn);
RasterPos3i::load_with(&mut *loadfn);
RasterPos3iv::load_with(&mut *loadfn);
RasterPos3s::load_with(&mut *loadfn);
RasterPos3sv::load_with(&mut *loadfn);
RasterPos4d::load_with(&mut *loadfn);
RasterPos4dv::load_with(&mut *loadfn);
RasterPos4f::load_with(&mut *loadfn);
RasterPos4fv::load_with(&mut *loadfn);
RasterPos4i::load_with(&mut *loadfn);
RasterPos4iv::load_with(&mut *loadfn);
RasterPos4s::load_with(&mut *loadfn);
RasterPos4sv::load_with(&mut *loadfn);
ReadBuffer::load_with(&mut *loadfn);
ReadPixels::load_with(&mut *loadfn);
Rectd::load_with(&mut *loadfn);
Rectdv::load_with(&mut *loadfn);
Rectf::load_with(&mut *loadfn);
Rectfv::load_with(&mut *loadfn);
Recti::load_with(&mut *loadfn);
Rectiv::load_with(&mut *loadfn);
Rects::load_with(&mut *loadfn);
Rectsv::load_with(&mut *loadfn);
RenderMode::load_with(&mut *loadfn);
Rotated::load_with(&mut *loadfn);
Rotatef::load_with(&mut *loadfn);
Scaled::load_with(&mut *loadfn);
Scalef::load_with(&mut *loadfn);
Scissor::load_with(&mut *loadfn);
SelectBuffer::load_with(&mut *loadfn);
ShadeModel::load_with(&mut *loadfn);
StencilFunc::load_with(&mut *loadfn);
StencilMask::load_with(&mut *loadfn);
StencilOp::load_with(&mut *loadfn);
TexCoord1d::load_with(&mut *loadfn);
TexCoord1dv::load_with(&mut *loadfn);
TexCoord1f::load_with(&mut *loadfn);
TexCoord1fv::load_with(&mut *loadfn);
TexCoord1i::load_with(&mut *loadfn);
TexCoord1iv::load_with(&mut *loadfn);
TexCoord1s::load_with(&mut *loadfn);
TexCoord1sv::load_with(&mut *loadfn);
TexCoord2d::load_with(&mut *loadfn);
TexCoord2dv::load_with(&mut *loadfn);
TexCoord2f::load_with(&mut *loadfn);
TexCoord2fv::load_with(&mut *loadfn);
TexCoord2i::load_with(&mut *loadfn);
TexCoord2iv::load_with(&mut *loadfn);
TexCoord2s::load_with(&mut *loadfn);
TexCoord2sv::load_with(&mut *loadfn);
TexCoord3d::load_with(&mut *loadfn);
TexCoord3dv::load_with(&mut *loadfn);
TexCoord3f::load_with(&mut *loadfn);
TexCoord3fv::load_with(&mut *loadfn);
TexCoord3i::load_with(&mut *loadfn);
TexCoord3iv::load_with(&mut *loadfn);
TexCoord3s::load_with(&mut *loadfn);
TexCoord3sv::load_with(&mut *loadfn);
TexCoord4d::load_with(&mut *loadfn);
TexCoord4dv::load_with(&mut *loadfn);
TexCoord4f::load_with(&mut *loadfn);
TexCoord4fv::load_with(&mut *loadfn);
TexCoord4i::load_with(&mut *loadfn);
TexCoord4iv::load_with(&mut *loadfn);
TexCoord4s::load_with(&mut *loadfn);
TexCoord4sv::load_with(&mut *loadfn);
TexCoordPointer::load_with(&mut *loadfn);
TexEnvf::load_with(&mut *loadfn);
TexEnvfv::load_with(&mut *loadfn);
TexEnvi::load_with(&mut *loadfn);
TexEnviv::load_with(&mut *loadfn);
TexGend::load_with(&mut *loadfn);
TexGendv::load_with(&mut *loadfn);
TexGenf::load_with(&mut *loadfn);
TexGenfv::load_with(&mut *loadfn);
TexGeni::load_with(&mut *loadfn);
TexGeniv::load_with(&mut *loadfn);
TexImage1D::load_with(&mut *loadfn);
TexImage2D::load_with(&mut *loadfn);
TexParameterf::load_with(&mut *loadfn);
TexParameterfv::load_with(&mut *loadfn);
TexParameteri::load_with(&mut *loadfn);
TexParameteriv::load_with(&mut *loadfn);
TexSubImage1D::load_with(&mut *loadfn);
TexSubImage2D::load_with(&mut *loadfn);
Translated::load_with(&mut *loadfn);
Translatef::load_with(&mut *loadfn);
Vertex2d::load_with(&mut *loadfn);
Vertex2dv::load_with(&mut *loadfn);
Vertex2f::load_with(&mut *loadfn);
Vertex2fv::load_with(&mut *loadfn);
Vertex2i::load_with(&mut *loadfn);
Vertex2iv::load_with(&mut *loadfn);
Vertex2s::load_with(&mut *loadfn);
Vertex2sv::load_with(&mut *loadfn);
Vertex3d::load_with(&mut *loadfn);
Vertex3dv::load_with(&mut *loadfn);
Vertex3f::load_with(&mut *loadfn);
Vertex3fv::load_with(&mut *loadfn);
Vertex3i::load_with(&mut *loadfn);
Vertex3iv::load_with(&mut *loadfn);
Vertex3s::load_with(&mut *loadfn);
Vertex3sv::load_with(&mut *loadfn);
Vertex4d::load_with(&mut *loadfn);
Vertex4dv::load_with(&mut *loadfn);
Vertex4f::load_with(&mut *loadfn);
Vertex4fv::load_with(&mut *loadfn);
Vertex4i::load_with(&mut *loadfn);
Vertex4iv::load_with(&mut *loadfn);
Vertex4s::load_with(&mut *loadfn);
Vertex4sv::load_with(&mut *loadfn);
VertexPointer::load_with(&mut *loadfn);
Viewport::load_with(&mut *loadfn);

            }

            inner(&mut loadfn)
        }
    
