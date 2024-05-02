
extern crate freetype;

use std::fmt::Pointer;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::Instant;
use freetype::face::LoadFlag;
use freetype::{RenderMode, Vector};
use gl::*;
use gl11::{MODELVIEW_MATRIX, PopMatrix, PushMatrix, Scaled};
use gl11::types::GLdouble;
use gl::types::{GLfloat, GLint, GLuint};
use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::WindowM;

const FONT_RES: u32 = 64u32;

pub struct Font<'a> {
    glyphs: Vec<Glyph>,
    renderer: &'a Renderer,
    size: f32,
    scale: f32,
    i_scale: f32,
    shader: Shader,
}

impl<'a> Font<'a> {
    pub unsafe fn new(font_path: &str, size: f32, renderer: &'a Renderer) -> Self {
        let mut font = Font {
            glyphs: Vec::new(),
            renderer,
            size,
            scale: size/FONT_RES as f32,
            i_scale: 1.0/(size/FONT_RES as f32),
            shader:
                Shader::new(read_to_string("src\\resources\\shaders\\sdf\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\sdf\\fragment.glsl").unwrap()),
        };

        let lib = freetype::Library::init().unwrap();
        let face = lib.new_face(font_path, 0).unwrap();

        face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();

        // let mut all_bytes: Vec<u8> = Vec::new();
        let mut all_bytes = std::fs::read("src\\resources\\fonts\\test.txt").unwrap();
        // println!("LEN: {}" )
        //std::fs::read("src\\resources\\fonts\\test.txt").unwrap()
        PixelStorei(UNPACK_ALIGNMENT, 1);
        let mut pos = 0;
        for i in 0..128 {
            face.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = face.glyph();

            // Maybe try pre-rendering before this loop
            // glyph.render_glyph(RenderMode::Sdf).unwrap();

            let mut tex_id = 0;
            GenTextures(1, &mut tex_id);
            BindTexture(TEXTURE_2D, tex_id);

            // Will need to store width and height in the file, likely just as 2 i32s before the data
            // let mut width = glyph.bitmap().width();
            // let mut height= glyph.bitmap().rows();
            let mut width = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let mut height= i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());

            // println!("{} {}", width, height);

            let len = width*height;

            // all_bytes.write(&width.to_be_bytes()).unwrap();
            // all_bytes.write(&height.to_be_bytes()).unwrap();
            // all_bytes.write_all(glyph.bitmap().buffer()).unwrap();

            let mut glyph_dat = Vec::new();
            // println!("{} {} {} {} {}", pos, len, glyph.bitmap().buffer().len(), width, height);
            for j in all_bytes.drain(..len as usize) {
                glyph_dat.push(j);
            }
            TexImage2D(
                TEXTURE_2D,
                0,
                ALPHA as GLint,
                width as i32,
                height as i32,
                0,
                ALPHA,
                UNSIGNED_BYTE,
                // glyph.bitmap().buffer().as_ptr().cast(),
                glyph_dat.as_ptr().cast(),
            );
            pos += len;

            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as GLint);
            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as GLint);
            TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
            TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);

            font.glyphs.push(Glyph {
                texture_id: tex_id,
                width,
                height,
                advance: (glyph.advance().x >> 6) as f32,
                bearing_x: (glyph.metrics().horiBearingX >> 6) as f32,
                top: (glyph.bitmap_top()) as f32,
            })
        }
        BindTexture(TEXTURE_2D, 0);
        // std::fs::write("src\\resources\\fonts\\test.txt", all_bytes).unwrap();

        font
    }

    // fn create_glyph_texture(width: i32, height: i32, bytes)

    pub unsafe fn draw_string_s(&self, string: &str, mut x: f32, mut y: f32, scale: f32, color: u32) {
        gl11::Enable(gl11::BLEND);
        x = x*self.i_scale;
        y = y*self.i_scale;

        PushMatrix();
        Scaled(self.scale as GLdouble, self.scale as GLdouble, 1 as GLdouble);

        self.shader.bind();
        self.shader.put_float("u_color", self.renderer.get_rgb(color));
        // self.shader.put_float("u_smoothing", vec![((1.0/self.size)*4.0*(FONT_RES as f32)/64.0)]);
        self.shader.put_float("u_smoothing", vec![0.28 / (self.size/10.0*scale)]);

        let str_height = self.glyphs.get('H' as usize).unwrap().top * self.scale;

        for char in string.chars() {
            let glyph: &Glyph = self.glyphs.get(char as usize).unwrap();

            BindTexture(TEXTURE_2D, glyph.texture_id);
            let pos_y = y + (str_height*self.i_scale) - glyph.top;
            self.renderer.draw_texture_rect(x, pos_y, x+glyph.width as f32, pos_y+glyph.height as f32, color);

            x += glyph.advance - glyph.bearing_x;
        }

        self.shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        gl11::Disable(gl11::BLEND);
    }

    pub unsafe fn draw_string(&self, string: &str, mut x: f32, mut y: f32, color: u32) {
        self.draw_string_s(string, x, y, 1.0, color);
    }
}

#[derive(Debug)]
struct Glyph {
    texture_id: GLuint,
    width: i32,
    height: i32,
    advance: f32,
    bearing_x: f32,
    top: f32,
}