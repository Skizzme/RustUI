
extern crate freetype;

use std::collections::HashMap;
use std::fmt::Pointer;
use std::fs::read_to_string;
use std::io::Write;
use std::mem::{size_of, size_of_val};
use std::path::Path;
use std::ptr;
use std::ptr::null;
use std::rc::Rc;
use std::time::{Instant};
use freetype::face::LoadFlag;
use freetype::{RenderMode};
// use gl::*;
// use gl::types::{GLint, GLuint};
use image::{GrayAlphaImage, ImageBuffer};
use crate::gl30::*;
use crate::gl30::types::{GLdouble, GLsizeiptr};
use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::window::Window;

const FONT_RES: u32 = 64u32;

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
        pub screen_width: i32,
        pub screen_height: i32,
}

impl FontManager {
    pub fn new(screen_width: i32, screen_height: i32, renderer: Rc<Renderer>) -> Self {
        FontManager {
            fonts: HashMap::new(),
                renderer: renderer.clone(),
                screen_width,
                screen_height,
        }
    }

    pub unsafe fn get_font(&mut self, name: &str) -> FontRenderer {
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
        FontRenderer::new(self, self.fonts.get(name).unwrap().clone())
    }

    pub fn updated_screen_dims(&mut self, width: i32, height: i32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}

// Only holds the data per font to be used by the font renderer
pub struct Font {
    glyphs: [Glyph; 128],
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
            glyphs: [Glyph::default(); 128],
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

            font.glyphs[i] =
                Glyph {
                atlas_x: atlas_width,
                    width,
                    height,
                    advance,
                    bearing_x,
                    top,
            };

            atlas_width += width;
        }

        let atlas_tex = Texture::create(renderer.clone(), atlas_width, atlas_height, all_bytes, ALPHA);
        font.atlas_tex = Some(atlas_tex);
        BindTexture(TEXTURE_2D, 0);

        font
    }
}

pub struct FontRenderer<'a> {
font: Rc<Font>,
    wrapping: Wrapping,
    scale_mode: ScaleMode,
    tab_length: u32, // The length of tabs in spaces. Default is 4
    line_spacing: f32,
    manager: &'a FontManager,
}

impl<'a> FontRenderer<'a> {

    pub unsafe fn new(manager: &'a FontManager, font: Rc<Font>) -> Self {
    FontRenderer {
        font: font,
        tab_length: 4,
        line_spacing: 1.0,
        wrapping: Wrapping::None,
        scale_mode: ScaleMode::Normal,
        manager,
    }
}

pub unsafe fn draw_string(&self, size: f32, string: &str, mut x: f32, mut y: f32, color: u32) -> (f32, f32) {
    let scale = size/FONT_RES as f32;
    let i_scale = 1.0/scale;

    let atlas = self.font.atlas_tex.as_ref().unwrap();
    Enable(BLEND);
    x = x*i_scale;
    y = y*i_scale;
    let start_x = x;

    let mut model_view_projection_matrix: [f64; 16] = [0.0; 16];
    GetDoublev(PROJECTION_MATRIX, model_view_projection_matrix.as_mut_ptr());
    let scaled_factor_x = (model_view_projection_matrix[0]*self.manager.screen_width as f64/2.0) as f32;
    let scaled_factor_y = (model_view_projection_matrix[5]*self.manager.screen_height as f64/-2.0) as f32;
    PushMatrix();
    Scaled(scale as GLdouble, scale as GLdouble, 1 as GLdouble);

    let str_height = self.font.glyphs.get('H' as usize).unwrap().top as f32;

    let mut line_width = 0f32;
    let mut line_height = 0f32;

    self.font.shader.bind();

    atlas.bind();
    self.font.shader.u_put_float("u_color", self.font.renderer.get_rgb(color));
    self.font.shader.u_put_float("u_values",
        vec![
        (0.25 / (size/10.0 * (scaled_factor_x+scaled_factor_y) as f32/2.0) * FONT_RES as f32/64.0).clamp(0.0, 0.4),
        atlas.width as f32,
        scaled_factor_x
]);
    // self.font.shader.u_put_float("u_smoothing", vec![(0.25 / (size/10.0 * scaled_factor) * FONT_RES as f32/64.0).clamp(0.0, 0.4)]);
    // self.font.shader.u_put_float("atlas_width", vec![atlas.width as f32]);
    // self.font.shader.u_put_float("i_scale", vec![i_scale]);
    for char in string.chars() {
        if char == '\n' {
            // TODO: maybe make these scale mods?
            match self.scale_mode {
                ScaleMode::Normal => {
                    y += line_height;
                }
                ScaleMode::Quality => {
                    y += (line_height * self.line_spacing * scale * scaled_factor_y).ceil() * i_scale * 1.0/scaled_factor_y;
                }
            }
            line_width = 0.0;
            x = start_x;
            continue;
        }
        if char == '\t' {
            x += self.get_width(size, " ".to_string());
            continue;
        }

        let (c_w, c_h) = self.draw_char(scaled_factor_x, scaled_factor_y, atlas, char, x, y, str_height, color);

        if line_height < c_h {
            line_height = c_h;
        }
        line_width += c_w;
        match self.scale_mode {
            ScaleMode::Normal => {
                x += c_w;
            }
            ScaleMode::Quality => {
                x += (c_w * scale * scaled_factor_x).ceil() * i_scale * 1.0/scaled_factor_x;
            }
        }
    }

    self.font.shader.unbind();

    BindTexture(TEXTURE_2D, 0);
    PopMatrix();
    atlas.unbind();
    Disable(BLEND);

    (line_width*scale, line_height*scale)
}

fn get_scaled_value(&self, value: f32, scale_factor: f32) -> f32 {
    (value * scale_factor).ceil() / scale_factor
}

pub unsafe fn draw_char(&self, scaled_x: f32, scaled_y: f32, atlas: &Texture, char: char, x: f32, y: f32, str_height: f32, color: u32) -> (f32, f32) {
    let glyph: &Glyph = self.font.glyphs.get(char as usize).unwrap();
    let pos_y = y + str_height - glyph.top as f32;

    // self.font.renderer.draw_texture_rect_uv(
    //     x,
    //     pos_y,
    //     (x+glyph.width as f32).ceil(),
    //     (pos_y+glyph.height as f32).ceil(),
    //     glyph.atlas_x as f64 / atlas.width as f64,
    //     0f64,
    //     (glyph.atlas_x + glyph.width) as f64 / atlas.width as f64,
    //     glyph.height as f64 / atlas.height as f64,
    //     color
    // );

    self.font.renderer.draw_texture_rect_uv(
        x,
        pos_y,
        self.get_scaled_value(x+glyph.width as f32, scaled_x),
    self.get_scaled_value(pos_y+glyph.height as f32, scaled_y),
    glyph.atlas_x as f64 / atlas.width as f64,
        0f64,
        (glyph.atlas_x + glyph.width) as f64 / atlas.width as f64,
        glyph.height as f64 / atlas.height as f64,
        color
);

    (((glyph.advance - glyph.bearing_x) as f32).ceil(), (glyph.height as f32).ceil())
}

pub unsafe fn get_width(&self, size: f32, string: String) -> f32 {
    let scale = size/FONT_RES as f32;
    let i_scale = 1.0/(size/FONT_RES as f32);
    let mut width = 0.0f32;

    for char in string.chars() {
        let glyph =  self.font.glyphs.get(char as usize).unwrap();
        width += (glyph.advance - glyph.bearing_x) as f32;
    }

    width*scale
}

pub unsafe fn get_height(&self, size: f32) -> f32 {
    let scale = size/FONT_RES as f32;
    let i_scale = 1.0/(size/FONT_RES as f32);
    self.font.glyphs.get('H' as usize).unwrap().top as f32 * scale
}

pub fn line_spacing(mut self, spacing: f32) -> Self {
    self.line_spacing = spacing;
    self
}

pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
    self.wrapping = wrapping;
    self
}

pub fn scale_mode(mut self, scale_mode: ScaleMode) -> Self {
    self.scale_mode = scale_mode;
    self
}
}

// Wrapping to be used for rendering
// Will contain the length of each line
pub enum Wrapping {
    None,
    Hard(f32),
    Soft(f32),
    SoftHard(f32), // If the word is longer than the line length, it will be hard wrapped
}

// To choose between smooth scaling (for animations)
// or to preserve quality for small text
pub enum ScaleMode {
    Normal,
    Quality,
}

#[derive(Debug, Default, Clone, Copy)]
struct Glyph {
    atlas_x: i32,
        width: i32,
        height: i32,
        advance: i32,
        bearing_x: i32,
        top: i32,
}