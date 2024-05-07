
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
use image::{GrayAlphaImage, ImageBuffer};
use crate::gl20::{EnableClientState, TexCoordPointer, VertexPointer};
use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::texture::Texture;

const FONT_RES: u32 = 128u32;

#[derive(Debug)]
struct CacheGlyph {
    bytes: Vec<u8>,
    width: i32,
    height: i32,
    advance: i32,
    bearing_x: i32,
    top: i32,
}

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
    atlas_tex: Option<Texture>,
}

impl Font {
    pub unsafe fn cache(font_path: &str, cache_path: &str) {
        let lib = freetype::Library::init().unwrap();
        let face = lib.new_face(font_path, 0).unwrap();

        face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();

        let mut cache_glyphs = Vec::new();
        let mut max_height = 0;

        PixelStorei(UNPACK_ALIGNMENT, 1);
        for i in 0..128 {
            face.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = face.glyph();

            glyph.render_glyph(RenderMode::Sdf).unwrap();

            let mut width = glyph.bitmap().width();
            let mut height = glyph.bitmap().rows();
            if max_height < height {
                max_height = height;
            }
            let mut bytes = Vec::new();
            bytes.write(glyph.bitmap().buffer()).unwrap();

            cache_glyphs.push(
                CacheGlyph {
                    bytes,
                    width,
                    height,
                    advance: glyph.advance().x >> 6,
                    bearing_x: glyph.metrics().horiBearingX >> 6,
                    top: glyph.bitmap_top(),
                }
            )
        }

        let mut meta_data: Vec<u8> = Vec::new();
        for mut c_glyph in cache_glyphs.as_slice() {
            meta_data.write(&c_glyph.width.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.height.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.advance.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.bearing_x.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.top.to_be_bytes()).unwrap();
        }

        // Creates the single texture atlas with all glyphs,
        // since swapping textures for every character is slow.
        // Is also in a single row to waste less pixel space
        let mut atlas_bytes: Vec<u8> = Vec::new();
        for i in 0..max_height {
            // Will write a single row of each glyph's pixels in order
            // so that a proper texture can be created quicker when loading
            for mut c_glyph in cache_glyphs.as_slice() {
                let offset = i * c_glyph.width;
                // Checks if the current glyph is too short, and if it is it will fill the empty space
                if c_glyph.width*c_glyph.height <= offset {
                    for j in 0..c_glyph.width { atlas_bytes.push(0u8); }
                } else {
                    atlas_bytes.write(&c_glyph.bytes[offset as usize..(offset+c_glyph.width) as usize]).unwrap();
                }
            }
        }

        meta_data.write_all(atlas_bytes.as_slice()).unwrap();
        BindTexture(TEXTURE_2D, 0);
        std::fs::write(cache_path, meta_data).unwrap();
    }

    pub unsafe fn load(cached_path: &str, renderer: Rc<Renderer>) -> Self {
        let mut font = Font {
            glyphs: Vec::new(),
            renderer: renderer.clone(),
            shader:
                Shader::new(read_to_string("src\\resources\\shaders\\sdf\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\sdf\\fragment.glsl").unwrap()),
            atlas_tex: None,
        };

        let mut all_bytes = std::fs::read(cached_path).unwrap();
        let mut atlas_height = 0;
        let mut atlas_width = 0;

        PixelStorei(UNPACK_ALIGNMENT, 1);
        for i in 0..128 {
            let width = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let height= i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let advance = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let bearing_x = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
            let top = i32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());

            if atlas_height < height {
                atlas_height = height;
            }

            font.glyphs.push(
                Glyph {
                    atlas_x: atlas_width,
                    width,
                    height,
                    advance,
                    bearing_x,
                    top,
                }
            );

            atlas_width += width;
        }

        let atlas_tex = Texture::create(renderer.clone(), atlas_width, atlas_height, all_bytes, ALPHA);
        font.atlas_tex = Some(atlas_tex);
        BindTexture(TEXTURE_2D, 0);

        font
    }

    // unsafe fn create_glyph_texture(width: i32, height: i32, advance: i32, bearing_x: i32, top: i32, data: Vec<u8>, renderer: Rc<Renderer>) -> Glyph {
    //     let tex = Texture::create(renderer, width, height, data, ALPHA);
        // println!("{:?} {:?} {:?} {:?}", vertices, uvs, elements, size_of::<[f32; 2]>());

    // }

    pub unsafe fn draw_string_s(&self, size: f32, string: &str, mut x: f32, mut y: f32, scaled_factor: f32, color: u32) -> (f32, f32) {

        let scale = size/FONT_RES as f32;
        let i_scale = 1.0/scale;

        let atlas = self.atlas_tex.as_ref().unwrap();
        gl11::Enable(gl11::BLEND);
        x = x*i_scale;
        y = y*i_scale;
        let start_x = x;

        PushMatrix();
        Scaled(scale as GLdouble, scale as GLdouble, 1 as GLdouble);


        let str_height = self.glyphs.get('H' as usize).unwrap().top as f32;

        let mut line_width = 0f32;
        let mut line_height = 0f32;
        self.shader.bind();
        self.shader.u_put_float("u_color", self.renderer.get_rgb(color));
        let smoothing = (0.25 / (size/10.0 * scaled_factor) * FONT_RES as f32/64.0);
        self.shader.u_put_float("u_smoothing", vec![smoothing]);
        atlas.bind();
        for char in string.chars() {
            if char == '\n' {
                y += line_height;
                line_width = 0.0;
                x = start_x;
                continue;
            }
            if char == '\t' {
                x += self.get_width(size, " ".to_string());
                continue;
            }
            let glyph: &Glyph = self.glyphs.get(char as usize).unwrap();

            let pos_y = y + str_height - glyph.top as f32;

            self.renderer.draw_texture_rect_uv(
                x,
                pos_y,
                x+glyph.width as f32,
                pos_y+glyph.height as f32,
                glyph.atlas_x as f64 / atlas.width as f64,
                0f64,
                (glyph.atlas_x + glyph.width) as f64 / atlas.width as f64,
                glyph.height as f64 / atlas.height as f64,
                color);

            line_width += (glyph.advance - glyph.bearing_x) as f32;
            if line_height < glyph.height as f32 {
                line_height = glyph.height as f32;
            }
            x += (glyph.advance - glyph.bearing_x) as f32;
        }

        self.shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        atlas.unbind();
        gl11::Disable(gl11::BLEND);

        (line_width *scale, line_height *scale)
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
    atlas_x: i32,
    width: i32,
    height: i32,
    advance: i32,
    bearing_x: i32,
    top: i32,
}