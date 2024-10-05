extern crate freetype;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;

use freetype::face::LoadFlag;
use freetype::{GlyphMetrics, RenderMode};
use freetype::ffi::FT_Size_Metrics;
use gl::*;
use gl::types::GLdouble;

use crate::asset_manager;
use crate::components::context::{context};
use crate::components::render::bounds::Bounds;
use crate::components::render::color::ToColor;
use crate::components::render::renderer::{Renderer, RendererWrapped};
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl30::{PopMatrix, PushMatrix, Scaled};

const FONT_RES: u32 = 48u32;

#[derive(Debug, Eq, PartialEq)]
struct CacheGlyph {
    id: usize,
    bytes: Vec<u8>,
    width: i32,
    height: i32,
    advance: i32,
    bearing_x: i32,
    top: i32,
}
impl PartialOrd<Self> for CacheGlyph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.id > other.id {
            Some(Greater)
        } else if self.id < other.id {
            Some(Less)
        } else {
            Some(Equal)
        }
    }
}

impl Ord for CacheGlyph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        if self.id > other.id {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self where Self: Sized {
        if self.id < other.id {
            self
        } else {
            other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, Self: PartialOrd {
        todo!()
    }
}

pub struct FontManager {
    fonts: HashMap<String, Font>,
    fonts_location: String,
    cache_location: String,
    mem_atlas_cache: HashMap<String, Vec<u8>>,
    sdf_shader: Shader,
    font_byte_library: HashMap<String, Vec<u8>>,
}

impl FontManager {
    pub unsafe fn new(fonts_location: impl ToString, cache_location: impl ToString) -> Self {
        let st = Instant::now();
        let s = Shader::new(asset_manager::file_contents_str("shaders/sdf/vertex.glsl").unwrap(), asset_manager::file_contents_str("shaders/sdf/fragment.glsl").unwrap());
        println!("{}", st.elapsed().as_secs_f32());
        FontManager {
            fonts: HashMap::new(),
            fonts_location: fonts_location.to_string(),
            cache_location: cache_location.to_string(),
            mem_atlas_cache: HashMap::new(),
            sdf_shader: s,
            font_byte_library: HashMap::new(),
        }
    }

    /// Sets the byte-data for a font to be used by the loader so that fonts don't have to be files
    ///
    /// Should only be used on setup, and the memory will be freed once the font is loaded
    ///
    pub fn set_font_bytes(&mut self, name: impl ToString, font_bytes: Vec<u8>) -> &mut FontManager {
        self.font_byte_library.insert(name.to_string(), font_bytes);
        self
    }

    pub unsafe fn load_font(&mut self, name: impl ToString, from_cache: bool) -> Option<String> {
        let name = name.to_string();
        if !self.fonts.contains_key(&name) {
            if !self.font_byte_library.contains_key(&name) {
                return Some(format!("No font data for '{}' was set. Use the 'set_font_bytes' method", &name));
            }
            let mut b = Instant::now();
            let cache_path = format!("{}{}_{}.cache", self.cache_location, &name, FONT_RES);
            if !from_cache {
                if !self.mem_atlas_cache.contains_key(&name) {
                    self.mem_atlas_cache.insert(name.to_string(), Font::create_font_data(self.font_byte_library.remove(&name).unwrap()));
                }

                let ft = Font::load(self.mem_atlas_cache.get(&name).unwrap().clone());
                println!("Font '{}' took {:?} to render and load...", &name, b.elapsed());

                self.fonts.insert(name, ft);
            } else {
                if !Path::new(cache_path.as_str()).exists() {
                    Font::cache(self.font_byte_library.remove(&name).unwrap(), cache_path.as_str());
                    println!("Font '{}' took {:?} to cache...", name, b.elapsed());
                }

                b = Instant::now();
                let ft = Font::load_from_file(format!("{}_{}.cache", name, FONT_RES).as_str());
                println!("Font '{}' took {:?} to load...", name, b.elapsed());

                self.fonts.insert(name.to_string(), ft);
            }
        }
        return None
    }

    /// Creates a new FontRenderer object every call
    ///
    /// This should not be called every frame, but is just a way to create a fond renderer with the needed options
    ///
    /// `name` references the name specified when calling `set_font_bytes`
    pub unsafe fn get_font(&mut self, name: &str) -> Result<FontRenderer, String> {
        if !self.fonts.contains_key(name) {
            self.load_font(name, false);
        }
        Ok(FontRenderer::new(self.fonts.get_mut(name).unwrap()))
    }
}

pub struct FontMetrics {
    ascent: f32,
    decent: f32,
}

/// Only holds the data per font to be used by the font renderer
pub struct Font {
    pub glyphs: [Glyph; 128],
    metrics: FontMetrics,
    pub atlas_tex: Option<Texture>,
}

impl Font {
    /// Saves this font's atlas texture to a file
    pub unsafe fn cache(font_bytes: Vec<u8>, cache_path: &str) {
        std::fs::write(cache_path, Font::create_font_data(font_bytes)).unwrap();
    }

    /// Creates the atlas texture and char data as bytes by rendering each char using FreeType using multithreading for faster speeds.
    ///
    /// Calls to this should be minimized
    pub unsafe fn create_font_data(font_bytes: Vec<u8>) -> Vec<u8> {

        let mut cache_glyphs: Vec<CacheGlyph> = Vec::new();
        let mut max_height = 0;

        PixelStorei(UNPACK_ALIGNMENT, 1);

        let (sender, receiver) = channel();

        let thread_count = 32; // Seems to provide the best results
        let mut threads = Vec::new();
        let m_lib = freetype::Library::init().unwrap();

        let m_face = m_lib.new_memory_face(font_bytes.clone(), 0).unwrap();
        m_face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();

        let size_metrics = m_face.size_metrics().unwrap();

        println!("{} {} {}", m_face.num_faces(), m_face.num_charmaps(), m_face.num_glyphs());
        for j in 0..thread_count {
            let total_length = 128;
            let batch_size = total_length / thread_count;
            let offset = j*batch_size;
            let t_sender = sender.clone();
            let st = Instant::now();
            let t_bytes = font_bytes.clone();
            let thread = std::thread::spawn(move || {
                let lib = freetype::Library::init().unwrap();
                let face = lib.new_memory_face(t_bytes, 0).unwrap();

                face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();
                for i in offset..offset + batch_size {
                    face.load_char(i, LoadFlag::RENDER).unwrap();

                    let glyph = face.glyph();

                    glyph.render_glyph(RenderMode::Sdf).unwrap();

                    let width = glyph.bitmap().width();
                    let height = glyph.bitmap().rows();

                    let mut bytes = Vec::new();
                    bytes.write(glyph.bitmap().buffer()).unwrap();

                    // println!("{:?} {:?} {:?}", i as u8 as char, glyph.metrics().vertBearingX >> 6, glyph.metrics().vertBearingY >> 6);
                    match t_sender.send(CacheGlyph {
                        id: i,
                        bytes,
                        width,
                        height,
                        advance: glyph.advance().x >> 6,
                        bearing_x: glyph.metrics().horiBearingX >> 6,
                        top: glyph.bitmap_top(),
                    }) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                        }
                    };
                }
            });
            threads.push(thread);
        }

        loop {
            let mut finished_count = 0;
            for i in 0..thread_count {
                if !threads[i].is_finished() {
                    break;
                }
                finished_count += 1;
            }
            if let Ok(recv) = receiver.try_recv() {
                cache_glyphs.push(recv);
            }
            if finished_count == thread_count {
                break;
            }
        }

        let mut meta_data: Vec<u8> = Vec::new();
        meta_data.write(&(size_metrics.ascender as f32 / 64.0).to_be_bytes()).unwrap();
        meta_data.write(&(size_metrics.descender as f32 / 64.0).to_be_bytes()).unwrap();
        cache_glyphs.sort();
        for c_glyph in cache_glyphs.as_slice() {
            if max_height < c_glyph.height {
                max_height = c_glyph.height;
            }
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
            for c_glyph in cache_glyphs.as_slice() {
                let offset = i * c_glyph.width;
                // Checks if the current glyph is too short/not enough height, and if it is it will fill the empty space
                if c_glyph.width*c_glyph.height <= offset {
                    for _ in 0..c_glyph.width { atlas_bytes.push(0u8); }
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
    /// use RustUI::components::render::font::FontManager;
    /// FontManager::load(std::fs::read("cached_path").unwrap(), /*args_here*/);
    /// ```
    pub unsafe fn load_from_file(cached_path: &str) -> Self {
        let all_bytes = std::fs::read(cached_path).unwrap();
        Self::load(all_bytes)
    }

    /// Loads each char / glyph from the same format created by [Font::create_font_data]
    pub unsafe fn load(mut all_bytes: Vec<u8>) -> Self {
        let ascent = f32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());
        let decent = f32::from_be_bytes(all_bytes.drain(..4).as_slice().try_into().unwrap());

        let mut font = Font {
            glyphs: [Glyph::default(); 128],
            metrics: FontMetrics {
                ascent,
                decent,
            },
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

        let atlas_tex = Texture::create(atlas_width, atlas_height, &all_bytes, ALPHA);
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
    pub font: &'a mut Font,
    pub wrapping: Wrapping,
    pub scale_mode: ScaleMode,
    pub tab_length: u32, // The length of tabs in spaces. Default is 4
    pub line_spacing: f32,

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
    pub unsafe fn new(font: &'a mut Font) -> Self {
        FontRenderer {
            font,
            tab_length: 4,
            line_spacing: 1.0,
            wrapping: Wrapping::None,
            scale_mode: ScaleMode::Normal,
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

    pub unsafe fn set_color(&mut self, color: impl ToColor) {
        let color = color.to_color();
        context().fonts().sdf_shader.u_put_float("u_color", color.rgba());
    }

    /// Renders a string using immediate GL
    ///
    /// The center of the rendered string is at `x`
    pub unsafe fn draw_centered_string(&mut self, size: f32, string: impl ToString, x: f32, y: f32, color: impl ToColor) -> (f32, f32) {
        let string = string.to_string();
        let width = self.get_width(size, string.clone());
        self.draw_string(size, string, x-width/2.0, y, color)
    }

    /// The method to be called to a render a string using immediate GL
    pub unsafe fn draw_string(&mut self, size: f32, string: impl ToString, x: f32, y: f32, color: impl ToColor) -> (f32, f32) {
        // let str_height = self.font.glyphs.get('H' as usize).unwrap().top as f32;
        self.begin(size, x, y);
        self.set_color(color);
        for char in string.to_string().chars() {
            if char == '\n' {
                match self.scale_mode {
                    ScaleMode::Normal => {
                        self.y += self.get_line_height() * self.comb_scale_y;
                    }
                    ScaleMode::Quality => {
                        self.y += self.get_scaled_value(self.get_line_height(), self.comb_scale_y);
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

            let (c_w, _c_h, should_render) = self.get_dimensions(char);
            if should_render == 2 {
                break;
            }

            if should_render <= 1 {
                if should_render == 0 {
                    let atlas_ref= self.font.atlas_tex.as_ref().unwrap().clone();
                    self.draw_char(self.comb_scale_x, self.comb_scale_y, &atlas_ref, char, self.x, self.y);
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
        (self.line_width*self.scale, self.get_line_height()*self.scale)
    }

    // todo make this match scale mode
    pub fn get_scaled_value(&self, value: f32, scale_factor: f32) -> f32 {
        match self.scale_mode {
            ScaleMode::Normal => (value * scale_factor) / scale_factor,
            ScaleMode::Quality => (value * scale_factor).ceil() / scale_factor
        }
    }

    /// Returns the necessary dimensions of a glyph / character
    ///
    /// Returns `char_width, char_height, should_render`
    ///
    /// `should_render` is an integer that is 0, 1, or 2. Is calculated based off of this FontRenderer's current offsets
    /// ```
    /// use RustUI::components::render::font::FontRenderer;
    /// let should_render = unsafe { FontRenderer::get_dimensions(FontRenderer::default() /*should be called non-statically*/, 'A') }.2;
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
    pub unsafe fn get_dimensions(&self, char: char) -> (f32, f32, u32) {
        let glyph: &Glyph = match self.font.glyphs.get(char as usize) {
            None => {
                return (0.0, 0.0, 0);
            }
            Some(glyph) => {
                glyph
            }
        };

        let (c_w, c_h) = match self.scale_mode {
            ScaleMode::Normal => ((glyph.advance - glyph.bearing_x) as f32, glyph.height as f32),
            ScaleMode::Quality => (((glyph.advance - glyph.bearing_x) as f32).ceil(), (glyph.height as f32).ceil())
        };
        let mut should_render = 0u32;
        if self.y > context().window().width as f32 * self.i_scale {
            should_render = 2;
        }
        else if self.y > -c_h {
            should_render = 0;
        }
        else if self.x <= context().window().height as f32 * self.i_scale {
            should_render = 1;
        }
        (c_w, c_h, should_render)
    }

    /// Draws a single char
    ///
    /// The exact draw methods are determined by this FontRenderer's options, like [FontRenderer::scale_mode] etc
    pub unsafe fn draw_char(&mut self, scaled_x: f32, scaled_y: f32, atlas: &Texture, char: char, x: f32, y: f32) -> (f32, f32) {
        let glyph: &Glyph = match self.font.glyphs.get(char as usize) {
            None => {
                return (0.0, 0.0);
            }
            Some(glyph) => {
                glyph
            }
        };
        let pos_y = y + self.get_height() - glyph.top as f32;

        let (right, bottom) = match self.scale_mode {
            ScaleMode::Normal => {
                (x+glyph.width as f32, pos_y+glyph.height as f32)
            }
            ScaleMode::Quality => {
                (self.get_scaled_value(x+glyph.width as f32, scaled_x), self.get_scaled_value(pos_y+glyph.height as f32, scaled_y))
            }
        };
        context().renderer().draw_texture_rect_uv(
            &Bounds::ltrb(x+glyph.bearing_x as f32, pos_y, right, bottom),
            &Bounds::ltrb(glyph.atlas_x as f32 / atlas.width as f32, 0f32, (glyph.atlas_x + glyph.width) as f32 / atlas.width as f32, glyph.height as f32 / atlas.height as f32),
            0xffffff,
        );
        // TODO make rendering use bearing x correctly
        match self.scale_mode {
            ScaleMode::Normal => (((glyph.advance - glyph.bearing_x) as f32), (glyph.height as f32)),
            ScaleMode::Quality => (((glyph.advance - glyph.bearing_x) as f32).floor(), (glyph.height as f32).floor())
        }
    }

    /// Sets this FontRenderer up for immediate GL drawing, setting shader uniforms, x and y offsets, scaling etc
    pub unsafe fn begin(&mut self, size: f32, x: f32, y: f32) {
        self.scale = match self.scale_mode {
            ScaleMode::Normal => {size/FONT_RES as f32}
            ScaleMode::Quality => {size.ceil()/FONT_RES as f32}
        };
        self.i_scale = 1.0/self.scale;

        let atlas = self.font.atlas_tex.as_ref().unwrap();
        println!("begin fr");
        context().renderer().stack().begin();
        context().renderer().stack().push(Blend(true));
        context().renderer().stack().push(Texture2D(true));
        self.x = x*self.i_scale;
        self.y = y*self.i_scale;
        self.start_x = self.x;

        let matrix: [f64; 16] = context().renderer().get_transform_matrix();
        self.scaled_factor_x = (matrix[0]*context().window().width as f64/2.0) as f32;
        self.scaled_factor_y = (matrix[5]*context().window().height as f64/-2.0) as f32;
        self.comb_scale_x = self.scaled_factor_x*self.scale;
        self.comb_scale_y = self.scaled_factor_y*self.scale;

        PushMatrix();
        Scaled(self.scale as GLdouble, self.scale as GLdouble, 1 as GLdouble);

        self.line_width = 0f32;

        context().fonts().sdf_shader.bind();
        atlas.bind();
        // was 0.25 / ... but .35 seems better?
        //(0.30 / (size / 9.0 *self.scaled_factor_x.max(self.scaled_factor_y)) * FONT_RES as f32 / 64.0).clamp(0.0, 0.4) // original smoothing
        let smoothing = (0.35 / (size / 6.0 *self.scaled_factor_x.max(self.scaled_factor_y)) * FONT_RES as f32 / 64.0).clamp(0.0, 0.25);
        context().fonts().sdf_shader.u_put_float("u_smoothing", vec![smoothing]);
        context().fonts().sdf_shader.u_put_float("atlas_width", vec![atlas.width as f32]);
        context().fonts().sdf_shader.u_put_float("i_scale", vec![1.0/self.comb_scale_x]);

        context().renderer().stack().end();
    }

    pub unsafe fn end(&self) {
        context().fonts().sdf_shader.unbind();

        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        self.font.atlas_tex.as_ref().unwrap().unbind();
        Disable(BLEND);
    }

    /// Returns the width, in pixels, of a string at a specific size
    pub unsafe fn get_width(&self, size: f32, string: String) -> f32 {
        let scale = size/FONT_RES as f32;
        let mut width = 0.0f32;

        for char in string.chars() {
            let glyph =  self.font.glyphs.get(char as usize).unwrap();
            width += (glyph.advance - glyph.bearing_x) as f32;
        }

        width*scale
    }

    /// Returns the height, in pixels, of the font. Unscaled
    pub unsafe fn get_height(&self) -> f32 {
        self.font.metrics.ascent + self.font.metrics.decent
        // self.font.glyphs.get('H' as usize).unwrap().top as f32 * scale
    }

    pub unsafe fn get_line_height(&self) -> f32 {
        self.get_height() + self.line_spacing
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