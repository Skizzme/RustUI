extern crate freetype;

use std::collections::HashMap;
use std::fmt::Pointer;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use freetype::face::LoadFlag;
use freetype::RenderMode;
use crate::gl30::{PopMatrix, PROJECTION_MATRIX, PushMatrix, Scaled};

use gl::*;
use gl::types::GLdouble;
use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::texture::Texture;

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
    fonts_location: String,
    cache_location: String,
    mem_atlas_cache: HashMap<String, Vec<u8>>,
}

impl FontManager {
    pub fn new(screen_width: i32, screen_height: i32, renderer: Rc<Renderer>, fonts_location: impl ToString, cache_location: impl ToString) -> Self {
        FontManager {
            fonts: HashMap::new(),
            renderer: renderer.clone(),
            screen_width,
            screen_height,
            fonts_location: fonts_location.to_string(),
            cache_location: cache_location.to_string(),
            mem_atlas_cache: HashMap::new(),
        }
    }


    /// Creates a new FontRenderer object every call
    ///
    /// This should not be called every frame, but is just a way to create a fond renderer with the needed options
    pub unsafe fn get_font(&mut self, name: &str, from_file_cache: bool) -> FontRenderer {
        if !self.fonts.contains_key(name) {
            let mut b = Instant::now();
            let cache_path = format!("{}{}_{}.cache", self.cache_location, name, FONT_RES);
            let font_path = format!("{}{}.ttf", self.fonts_location, name);
            if !from_file_cache {
                if !self.mem_atlas_cache.contains_key(name) {
                    self.mem_atlas_cache.insert(name.to_string(), Font::create_font_data(&font_path));
                }

                let ft = Font::load(self.mem_atlas_cache.get(&name.to_string()).unwrap().clone(), self.renderer.clone());
                println!("Font took {:?} to render and load...", b.elapsed());

                self.fonts.insert(name.to_string(), Rc::new(ft));
            } else if !Path::new(cache_path.as_str()).exists() {
                Font::cache(font_path.as_str(), cache_path.as_str());
                println!("Font took {:?} to cache...", b.elapsed());

                b = Instant::now();
                let ft = Font::load_from_file(format!("{}_{}.cache", name, FONT_RES).as_str(), self.renderer.clone());
                println!("Font took {:?} to load...", b.elapsed());

                self.fonts.insert(name.to_string(), Rc::new(ft));
            }
        }
        FontRenderer::new(self, self.fonts.get(name).unwrap().clone())
    }

    pub fn updated_screen_dims(&mut self, width: i32, height: i32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}

/// Only holds the data per font to be used by the font renderer
pub struct Font {
    pub glyphs: [Glyph; 128],
    renderer: Rc<Renderer>,
    shader: Shader,
    pub atlas_tex: Option<Texture>,
}

impl Font {
    /// Saves this font's atlas texture to a file
    pub unsafe fn cache(font_path: &str, cache_path: &str) {
        std::fs::write(cache_path, Font::create_font_data(font_path)).unwrap();
    }

    /// Creates the atlas texture and char data as bytes by rendering each char using FreeType.
    ///
    /// Calls to this should be minimized
    pub unsafe fn create_font_data(font_path: &str) -> Vec<u8> {
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

        /// Creates the single texture atlas with all glyphs,
        /// since swapping textures for every character is slow.
        /// Is also in a single row to waste less pixel space
        let mut atlas_bytes: Vec<u8> = Vec::new();
        for i in 0..max_height {
            /// Will write a single row of each glyph's pixels in order
            /// so that a proper texture can be created quicker when loading
            for mut c_glyph in cache_glyphs.as_slice() {
                let offset = i * c_glyph.width;
                // Checks if the current glyph is too short/not enough height, and if it is it will fill the empty space
                if c_glyph.width*c_glyph.height <= offset {
                    for j in 0..c_glyph.width { atlas_bytes.push(0u8); }
                } else {
                    atlas_bytes.write(&c_glyph.bytes[offset as usize..(offset+c_glyph.width) as usize]).unwrap();
                }
            }
        }

        meta_data.write_all(atlas_bytes.as_slice()).unwrap();
        BindTexture(TEXTURE_2D, 0);
        meta_data
    }

    /// A shortcut to calling
    ///
    /// ```
    /// Font::load(std::fs::read("cached_path").unwrap(), renderer);
    /// ```
    pub unsafe fn load_from_file(cached_path: &str, renderer: Rc<Renderer>) -> Self {
        let mut all_bytes = std::fs::read(cached_path).unwrap();
        Self::load(all_bytes, renderer)
    }

    /// Loads each char / glyph from the same format created by [Font::create_font_data]
    pub unsafe fn load(mut all_bytes: Vec<u8>, renderer: Rc<Renderer>) -> Self {
        let mut font = Font {
            glyphs: [Glyph::default(); 128],
            renderer: renderer.clone(),
            shader:
                Shader::new(read_to_string("src\\resources\\shaders\\sdf\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\sdf\\fragment.glsl").unwrap()),
            atlas_tex: None,
        };

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

        let atlas_tex = Texture::create(renderer.clone(), atlas_width, atlas_height, &all_bytes, ALPHA);
        font.atlas_tex = Some(atlas_tex);
        BindTexture(TEXTURE_2D, 0);

        font
    }
}

/// The object used to render fonts
///
/// Contains options like tab length, line spacing, wrapping etc. for convenience
///
/// It would be preferable not to be created each frame
pub struct FontRenderer<'a> {
    pub font: Rc<Font>,
    pub wrapping: Wrapping,
    pub scale_mode: ScaleMode,
    pub tab_length: u32, // The length of tabs in spaces. Default is 4
    pub line_spacing: f32,
    pub manager: &'a FontManager,

    pub scaled_factor_x: f32,
    pub scaled_factor_y: f32,
    pub comb_scale_x: f32,
    pub comb_scale_y: f32,
    pub scale: f32,
    pub i_scale: f32,
    pub start_x: f32,
    pub x: f32,
    pub y: f32,
    pub line_width: f32,
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
            scaled_factor_x: 0.0,
            scaled_factor_y: 0.0,
            comb_scale_x: 0.0,
            comb_scale_y: 0.0,
            scale: 0.0,
            i_scale: 0.0,
            start_x: 0.0,
            x: 0.0,
            y: 0.0,
            line_width: 0.0,
        }
    }

    pub unsafe fn set_color(&mut self, color: u32) {
        self.font.shader.u_put_float("u_color", self.font.renderer.get_rgb(color));
    }

    /// Renders a string using immediate GL
    ///
    /// The center of the rendered string is at `x`
    pub unsafe fn draw_centered_string(&mut self, size: f32, string: impl ToString, mut x: f32, mut y: f32, color: u32) -> (f32, f32) {
        let string = string.to_string();
        let width = self.get_width(size, string.clone());
        self.draw_string(size, string, x-width/2.0, y, color)
    }

    /// The method to be called to a render a string using immediate GL
    pub unsafe fn draw_string(&mut self, size: f32, string: impl ToString, mut x: f32, mut y: f32, color: u32) -> (f32, f32) {
        self.begin(size, x, y);
        self.set_color(color);
        let str_height = self.font.glyphs.get('H' as usize).unwrap().top as f32;
        for char in string.to_string().chars() {
            if char == '\n' {
                match self.scale_mode {
                    ScaleMode::Normal => {
                        self.y += (str_height + 2.0);
                    }
                    ScaleMode::Quality => {
                        self.y += self.get_scaled_value((str_height + 2.0) * self.line_spacing, self.comb_scale_y);
                    }
                }
                self.line_width = 0.0;
                self.x = self.start_x;
                continue;
            }

            if char == '\t' {
                self.x += self.get_width(size, " ".to_string())*self.tab_length as f32;
                continue;
            }

            let (c_w, c_h, should_render) = self.get_dimensions(char);
            if should_render == 2 {
                break;
            }

            if should_render <= 1 {
                if should_render == 0 {
                    self.draw_char(self.comb_scale_x, self.comb_scale_y, self.font.atlas_tex.as_ref().unwrap(), char, self.x, self.y, str_height);
                }

                self.line_width += c_w;
                match self.scale_mode {
                    ScaleMode::Normal => {
                        self.x += c_w;
                    }
                    ScaleMode::Quality => {
                        self.x += self.get_scaled_value(c_w, self.comb_scale_x);
                    }
                }
            }
        }
        self.end();
        (self.line_width*self.scale, str_height*self.scale)
    }

    pub fn get_scaled_value(&self, value: f32, scale_factor: f32) -> f32 {
        (value * scale_factor).floor() / scale_factor
    }

    /// Returns the necessary dimensions of a glyph / character
    ///
    /// Returns `char_width, char_height, should_render`
    ///
    /// `should_render` is an integer that is 0, 1, or 2. Is calculated based off of this FontRenderer's current offsets
    /// ```
    /// use RustUI::font::FontRenderer;
    /// let should_render = font_renderer::get_dimensions('A').2;
    /// if should_render == 2 {
    ///     // End the rendering.
    ///     // This text is out of screen and no more will be rendered
    /// }
    /// if should_render <= 1 {
    ///     if should_render == 0 {
    ///         // Actually draw the char
    ///     }
    ///     // Calculate next positions, because here is either in screen, or out of screen.
    ///     // There will still be more characters to be rendered after this one
    /// }
    /// ```
    pub fn get_dimensions(&self, char: char) -> (f32, f32, u32) {
        let glyph: &Glyph = match self.font.glyphs.get(char as usize) {
            None => {
                return (0.0, 0.0, 0);
            }
            Some(glyph) => {
                glyph
            }
        };

        let (c_w, c_h) = (((glyph.advance - glyph.bearing_x) as f32).ceil(), (glyph.height as f32).ceil());
        let mut should_render = 0u32;
        if self.y > self.manager.screen_height as f32 * self.i_scale {
            should_render = 2;
        }
        else if self.y > -c_h {
            should_render = 0;
        }
        else if self.x <= self.manager.screen_width as f32 * self.i_scale {
            should_render = 1;
        }
        (c_w, c_h, should_render)
    }

    /// Draws a single char
    ///
    /// The exact draw methods are determined by this FontRenderer's options, like [FontRenderer::scale_mode] etc
    pub unsafe fn draw_char(&self, scaled_x: f32, scaled_y: f32, atlas: &Texture, char: char, x: f32, y: f32, offset_y: f32) -> (f32, f32) {
        let glyph: &Glyph = match self.font.glyphs.get(char as usize) {
            None => {
                return (0.0, 0.0);
            }
            Some(glyph) => {
                glyph
            }
        };
        let pos_y = y + offset_y - glyph.top as f32;

        let (right, bottom) = match self.scale_mode {
            ScaleMode::Normal => {
                (x+glyph.width as f32, pos_y+glyph.height as f32)
            }
            ScaleMode::Quality => {
                (self.get_scaled_value(x+glyph.width as f32, scaled_x), self.get_scaled_value(pos_y+glyph.height as f32, scaled_y))
            }
        };
        self.font.renderer.draw_texture_rect_uv(
            x,
            pos_y,
            right,
            bottom,
            glyph.atlas_x as f64 / atlas.width as f64,
            0f64,
            (glyph.atlas_x + glyph.width) as f64 / atlas.width as f64,
            glyph.height as f64 / atlas.height as f64,
            0xffffff,
        );

        (((glyph.advance - glyph.bearing_x) as f32).ceil(), (glyph.height as f32).ceil())
    }

    /// Sets this FontRenderer up for immediate GL drawing, setting shader uniforms, x and y offsets, scaling etc
    pub unsafe fn begin(&mut self, size: f32, mut x: f32, mut y: f32) {
        self.scale = match self.scale_mode {
            ScaleMode::Normal => {size/FONT_RES as f32}
            ScaleMode::Quality => {size.ceil()/FONT_RES as f32}
        };
        self.i_scale = 1.0/self.scale;

        let atlas = self.font.atlas_tex.as_ref().unwrap();
        Enable(BLEND);
        self.x = x*self.i_scale;
        self.y = y*self.i_scale;
        self.start_x = x;

        let mut model_view_projection_matrix: [f64; 16] = [0.0; 16];
        GetDoublev(PROJECTION_MATRIX, model_view_projection_matrix.as_mut_ptr());
        self.scaled_factor_x = (model_view_projection_matrix[0]*self.manager.screen_width as f64/2.0) as f32;
        self.scaled_factor_y = (model_view_projection_matrix[5]*self.manager.screen_height as f64/-2.0) as f32;
        self.comb_scale_x = self.scaled_factor_x*self.scale;
        self.comb_scale_y = self.scaled_factor_y*self.scale;

        PushMatrix();
        Scaled(self.scale as GLdouble, self.scale as GLdouble, 1 as GLdouble);

        self.line_width = 0f32;

        self.font.shader.bind();

        atlas.bind();
        self.font.shader.u_put_float("u_smoothing", vec![(0.25 / (size / 9.0 *self.scaled_factor_x.max(self.scaled_factor_y)) * FONT_RES as f32 / 64.0).clamp(0.0, 0.4)]);
        self.font.shader.u_put_float("atlas_width", vec![atlas.width as f32]);
        self.font.shader.u_put_float("i_scale", vec![1.0/self.comb_scale_x]);
    }

    pub unsafe fn end(&self) {
        self.font.shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        self.font.atlas_tex.as_ref().unwrap().unbind();
        Disable(BLEND);
    }

    /// Returns the width, in pixels, of a string at a specific size
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

    /// Returns the height, in pixels, of a string at a specific size
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

/// Wrapping to be used for rendering
///
/// When not [Wrapping::None], the enum should contain the maximum line length (in pixels)
pub enum Wrapping {
    /// No wrapping
    None,
    /// Will wrap at any character, and could split words up
    Hard(f32),
    /// Will wrap only at spaces. Will not break up words
    Soft(f32),
    /// Will try to wrap only at spaces, but if one word is longer than the maximum line length, it would resort to hard wrapping
    SoftHard(f32),
}

/// To choose between smooth scaling (for animations),
/// or to preserve quality / readability for small text
pub enum ScaleMode {
    /// No correction, and can be hard to read when scaled far below the normal size. Around size 8
    Normal,
    /// Forces the characters to stay aligned with pixels, and preserves readability at much smaller font sizes
    Quality,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Glyph {
    pub atlas_x: i32,
    pub width: i32,
    pub height: i32,
    pub advance: i32,
    pub bearing_x: i32,
    pub top: i32,
}