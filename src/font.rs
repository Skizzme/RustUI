
extern crate freetype;

use std::collections::HashMap;
use std::fmt::Pointer;
use std::fs::read_to_string;
use std::io::Write;
use std::mem::{size_of, size_of_val};
use std::path::Path;
use std::ptr::null;
use std::rc::Rc;
use std::time::{Instant};
use freetype::face::LoadFlag;
use freetype::{RenderMode};
use gl::*;
use gl11::{PopMatrix, PushMatrix, Scaled, Translatef};
use gl11::types::GLdouble;
use gl::types::{GLint, GLuint};
use crate::gl20::{EnableClientState, TexCoordPointer, VertexPointer};
use crate::renderer::Renderer;
use crate::shader::Shader;

const FONT_RES: u32 = 128u32;

pub struct FontManager {
    fonts: HashMap<String, Rc<Font>>,
    renderer: Rc<Renderer>,
}

impl FontManager {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        FontManager {
            fonts: HashMap::new(),
            renderer: renderer.clone(),
        }
    }

    pub unsafe fn get_font(&mut self, name: &str) -> Rc<Font> {
        if !self.fonts.contains_key(name) {
            let mut b = Instant::now();
            if !Path::new(format!("{}_{}.cache", name, FONT_RES).as_str()).exists() {
                Font::cache(format!("src\\resources\\fonts\\{}.ttf", name).as_str(), format!("{}_{}.cache", name, FONT_RES).as_str());
                println!("Font took {:?} to cache...", b.elapsed());
            }

            b = Instant::now();
            let ft = Font::load(format!("{}_{}.cache", name, FONT_RES).as_str(), self.renderer.clone());
            println!("Font took {:?} to load...", b.elapsed());

            self.fonts.insert(name.to_string(), Rc::new(ft));
        }
        self.fonts.get(name).unwrap().clone()
    }


}

pub struct Font {
    glyphs: Vec<Glyph>,
    renderer: Rc<Renderer>,
    shader: Shader,
}

impl Font {
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

    pub unsafe fn load(cached_path: &str, renderer: Rc<Renderer>) -> Self {
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

        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);

        let mut vao = 0;
        let mut vbo = 0;
        let mut uvo = 0;
        let mut ebo = 0;
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut uvo);
        GenBuffers(1, &mut ebo);
        BindVertexArray(vao);

        let vertices: [[f32; 2]; 4] =
            [[0.0, 0.0], [width as f32, 0.0], [width as f32, height as f32], [0.0, height as f32]];

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            STATIC_DRAW
        );

        let uvs: [[f32; 2]; 4] =
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

        BindBuffer(ARRAY_BUFFER, uvo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(&uvs) as isize,
            uvs.as_ptr().cast(),
            STATIC_DRAW
        );

        let elements = [0, 1, 2, 0, 2, 3];
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            size_of_val(&elements) as isize,
            elements.as_ptr().cast(),
            STATIC_DRAW
        );

        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, vbo);
        VertexAttribPointer(
            0,
            2,
            FLOAT,
            FALSE,
            size_of::<[f32; 2]>().try_into().unwrap(),
            0 as *const _,
        );

        EnableVertexAttribArray(1);
        BindBuffer(ARRAY_BUFFER, uvo);
        VertexAttribPointer(
            1,
            2,
            FLOAT,
            FALSE,
            size_of::<[f32; 2]>().try_into().unwrap(),
            0 as *const _,
        );

        // println!("{:?} {:?} {:?} {:?}", vertices, uvs, elements, size_of::<[f32; 2]>());

        Glyph {
            texture_id: tex_id,
            width,
            height,
            advance,
            bearing_x,
            top,
            vbo,
            vao,
            uvo,
            ebo,
        }
    }

    unsafe fn draw_char(&self, c: char) {
        gl11::Enable(gl11::TEXTURE_2D);
        let glyph: &Glyph = self.glyphs.get(c as usize).unwrap();

        BindVertexArray(glyph.vao);

        DrawElements(TRIANGLES, 6, UNSIGNED_BYTE, null());

        BindVertexArray(0);
    }

    pub unsafe fn draw_string_s(&self, size: f32, string: &str, mut x: f32, mut y: f32, scaled_factor: f32, color: u32) -> (f32, f32) {

        let scale = size/FONT_RES as f32;
        let i_scale = 1.0/scale;

        gl11::Enable(gl11::BLEND);
        x = x*i_scale;
        y = y*i_scale;

        PushMatrix();
        Scaled(scale as GLdouble, scale as GLdouble, 1 as GLdouble);


        let str_height = self.glyphs.get('H' as usize).unwrap().top as f32;

        let mut width = 0f32;
        let mut height = 0f32;

        for char in string.chars() {
            let glyph: &Glyph = self.glyphs.get(char as usize).unwrap();

            PushMatrix();
            let pos_y = y + str_height - glyph.top as f32;
            Translatef(x+width, pos_y, 0.0);
            self.shader.bind();
            self.shader.u_put_float("u_color", self.renderer.get_rgb(color));
            let smoothing = (0.25 / (size/10.0 * scaled_factor) * FONT_RES as f32/64.0).clamp(0.0, 0.6);
            self.shader.u_put_float("u_smoothing", vec![smoothing]);
            BindTexture(TEXTURE_2D, glyph.texture_id);
            self.renderer.draw_texture_rect(0.0, 0.0, glyph.width as f32, glyph.height as f32, color);
            // self.draw_char(char);

            width += (glyph.advance - glyph.bearing_x) as f32;
            if height < glyph.height as f32 {
                height = glyph.height as f32;
            }
            PopMatrix();
        }

        self.shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        gl11::Disable(gl11::BLEND);

        (width*scale, height*scale)
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

    pub unsafe fn draw_string(&self, size: f32, string: &str, mut x: f32, mut y: f32, color: u32) -> (f32, f32) {
        self.draw_string_s(size, string, x, y, 1.0, color)
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
    vao: GLuint,
    vbo: GLuint,
    uvo: GLuint,
    ebo: GLuint,
}