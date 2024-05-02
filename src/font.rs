
extern crate freetype;

use std::fmt::Pointer;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::Instant;
use freetype::face::LoadFlag;
use freetype::{GlyphSlot, RenderMode, Vector};
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
    shader: Shader,
}

impl<'a> Font<'a> {
    pub unsafe fn cache(font_path: &str, cache_path: &str) {
        let lib = freetype::Library::init().unwrap();
        let face = lib.new_face(font_path, 0).unwrap();

        face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();

        let mut all_bytes: Vec<u8> = Vec::new();
        PixelStorei(UNPACK_ALIGNMENT, 1);
        let mut pos = 0;
        for i in 0..128 {
            face.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = face.glyph();

            glyph.render_glyph(RenderMode::Sdf).unwrap();

            let mut width = glyph.bitmap().width();
            let mut height= glyph.bitmap().rows();

            let len = width*height;

            all_bytes.write(&width.to_be_bytes()).unwrap();
            all_bytes.write(&height.to_be_bytes()).unwrap();
            all_bytes.write(&(glyph.advance().x >> 6).to_be_bytes()).unwrap();
            all_bytes.write(&(glyph.metrics().horiBearingX >> 6).to_be_bytes()).unwrap();
            all_bytes.write(&glyph.bitmap_top().to_be_bytes()).unwrap();
            all_bytes.write_all(glyph.bitmap().buffer()).unwrap();

        }
        BindTexture(TEXTURE_2D, 0);
        std::fs::write(cache_path, all_bytes).unwrap();
    }

    pub unsafe fn load(cached_path: &str, renderer: &'a Renderer) -> Self {
        let mut font = Font {
            glyphs: Vec::new(),
            renderer,
            shader:
                Shader::new(read_to_string("src\\resources\\shaders\\sdf\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\sdf\\fragment.glsl").unwrap()),
        };

        let mut all_bytes = std::fs::read(cached_path).unwrap();

        PixelStorei(UNPACK_ALIGNMENT, 1);
        let mut pos = 0;
        for i in 0..128 {

            let width = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let height= i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let advance = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let bearing_x = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let top = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());

            let len = width*height;

            let mut glyph_dat = Vec::new();

            for j in all_bytes.drain(..len as usize) {
                glyph_dat.push(j);
            }

            font.glyphs.push(
                Self::create_glyph_texture(
                    width,
                    height,
                    advance,
                    bearing_x,
                    top,
                    glyph_dat.as_ptr()
                )
            );
            pos += len;
        }
        BindTexture(TEXTURE_2D, 0);

        font
    }

    unsafe fn create_glyph_texture(width: i32, height: i32, advance: i32, bearing_x: i32, top: i32, data: *const u8) -> Glyph {
        let mut tex_id = 0;
        GenTextures(1, &mut tex_id);
        BindTexture(TEXTURE_2D, tex_id);

        TexImage2D(
            TEXTURE_2D,
            0,
            ALPHA as GLint,
            width as i32,
            height as i32,
            0,
            ALPHA,
            UNSIGNED_BYTE,
            data.cast(),
        );

        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);

        Glyph {
            texture_id: tex_id,
            width,
            height,
            advance,
            bearing_x,
            top,
        }
    }


    pub unsafe fn draw_string_s(&self, size: f32, string: &str, mut x: f32, mut y: f32, scaled: f32, color: u32) {

        let scale = size/FONT_RES as f32;
        let i_scale = 1.0/(size/FONT_RES as f32);

        gl11::Enable(gl11::BLEND);
        x = x*i_scale;
        y = y*i_scale;

        PushMatrix();
        Scaled(scale as GLdouble, scale as GLdouble, 1 as GLdouble);

        self.shader.bind();
        self.shader.put_float("u_color", self.renderer.get_rgb(color));
        self.shader.put_float("u_smoothing", vec![0.3 / (size/10.0*scaled)]);

        let str_height = self.glyphs.get('H' as usize).unwrap().top as f32;

        for char in string.chars() {
            let glyph: &Glyph = self.glyphs.get(char as usize).unwrap();

            BindTexture(TEXTURE_2D, glyph.texture_id);
            let pos_y = y + str_height - glyph.top as f32;
            self.renderer.draw_texture_rect(x, pos_y, x+glyph.width as f32, pos_y+glyph.height as f32, color);

            x += (glyph.advance - glyph.bearing_x) as f32;
        }

        self.shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        gl11::Disable(gl11::BLEND);
    }

    pub unsafe fn get_width(&self, size: f32, string: String) -> f32 {
        let scale = size/FONT_RES as f32;
        let i_scale = 1.0/(size/FONT_RES as f32);
        let mut width = 0.0f32;

        for char in string.chars() {
            let glyph =  self.glyphs.get(char as usize).unwrap();
            width += (glyph.advance - glyph.bearing_x) as f32;
        }

        width*scale
    }

    pub unsafe fn get_height(&self, size: f32) -> f32 {
        let scale = size/FONT_RES as f32;
        let i_scale = 1.0/(size/FONT_RES as f32);
        self.glyphs.get('H' as usize).unwrap().top as f32 * scale
    }

    pub unsafe fn draw_string(&self, size: f32, string: &str, mut x: f32, mut y: f32, color: u32) {
        self.draw_string_s(size, string, x, y, 1.0, color);
    }
}

#[derive(Debug)]
struct Glyph {
    texture_id: GLuint,
    width: i32,
    height: i32,
    advance: i32,
    bearing_x: i32,
    top: i32,
}