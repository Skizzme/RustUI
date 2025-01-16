extern crate freetype;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::hash;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::time::Instant;
use freetype::{Library, RenderMode};
use freetype::face::LoadFlag;

use gl::*;
use crate::components::context::context;
use crate::components::render::color::Color;
use crate::components::render::font::format::{Alignment, FormatItem, Text};
use crate::components::render::stack::State::{Blend, Texture2D};

use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::types::{GLsizei, GLuint};

pub mod format;
pub mod manager;

const FONT_RES: u32 = 48u32;
const MAX_ATLAS_WIDTH: i32 = 2000;

#[derive(Debug, PartialEq, Eq)]
struct CacheGlyph {
    id: usize,
    bytes: Vec<u8>,
    atlas_pos: Vec2,
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
}


pub struct FontMetrics {
    ascent: f32,
    decent: f32,
}

#[derive( Debug)]
struct RenderData {
    scale: f32,
    i_scale: f32,
    x: f32,
    y: f32,
    start_x: f32,
    comb_scale_x: f32,
    comb_scale_y: f32,
    line_width: f32,

    current_color: Color,
    current_size: f32,
    current_offset: Vec2,
    current_align_h: f32,
    current_align_v: f32,
    current_tab_length: u32,
    current_line_spacing: f32,
}

impl Default for RenderData {
    fn default() -> Self {
        RenderData {
            scale: 1.,
            i_scale: 1.,
            x: 0.0,
            y: 0.0,
            start_x: 0.0,
            comb_scale_x: 1.0,
            comb_scale_y: 1.0,
            line_width: 0.0,
            current_color: Color::from_u32(0xffffffff),
            current_size: 16.0,
            current_offset: Default::default(),
            current_align_h: 0.0,
            current_align_v: 0.0,
            current_tab_length: 0,
            current_line_spacing: 1.5,
        }
    }
}

#[derive(Clone)]
pub struct FontRenderData {
    end_char_pos: Vec2,
    bounds: Vec4,

    char_positions: Rc<Vec<[f32; 4]>>,
    line_ranges: Rc<Vec<(usize, usize)>>,
}

impl FontRenderData {
    pub fn end_char_pos(&self) -> Vec2 {
        self.end_char_pos
    }
    pub fn bounds(&self) -> Vec4 {
        self.bounds
    }

    pub fn char_positions(&self) -> &Rc<Vec<[f32; 4]>> {
        &self.char_positions
    }
    pub fn line_ranges(&self) -> &Rc<Vec<(usize, usize)>> {
        &self.line_ranges
    }

    pub fn get_data_at(&self, column: usize, line: usize) -> [f32; 4] {
        let mut out = [0., 0., 0., 0.];
        match self.line_ranges.get(line) {
            None => {}
            Some((start, end)) => {
                let index = column + start;
                if index < *end {
                    out = self.char_positions.get(index).unwrap().clone();
                }
            }
        }
        out
    }

    pub fn get_data_at_index(&self, global_index: usize) -> [f32; 4] {
        let mut out = [0., 0., 0., 0.];
        let start_index = match self.line_ranges.first() {
            None => 0,
            Some(o) => o.0
        };
        let local_index = global_index - start_index;
        match self.char_positions.get(local_index) {
            None => {}
            Some(v) => out = v.clone(),
        }
        out
    }
}

/// Only holds the data per font to be used by the font renderer
pub struct Font {
    pub glyphs: HashMap<usize, Glyph>,
    metrics: FontMetrics,
    pub atlas_tex: Option<Texture>,

    draw_data: RenderData,
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

        let st = Instant::now();

        let m_lib = freetype::Library::init().unwrap();

        let m_face = m_lib.new_memory_face(font_bytes.clone(), 0).unwrap();
        m_face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();
        let all_chars = m_face.chars().map(|(c,i)| (c, i.get() as usize)).collect::<Vec<(usize, usize)>>();
        let num_glyphs = all_chars.len();

        let size_metrics = m_face.size_metrics().unwrap();
        for j in 0..thread_count {
            let batch_size = num_glyphs / thread_count;
            let offset = j*batch_size;
            let t_sender = sender.clone();
            let t_bytes = font_bytes.clone();
            let thread_chars = all_chars[offset..offset + batch_size].to_vec();
            let thread = std::thread::spawn(move || {
                let lib = Library::init().unwrap();

                let face = lib.new_memory_face(t_bytes, 0).unwrap();

                face.set_pixel_sizes(FONT_RES, FONT_RES).unwrap();
                for (char, index) in thread_chars {

                    face.load_char(char, LoadFlag::RENDER).unwrap();

                    let glyph = face.glyph();

                    glyph.render_glyph(RenderMode::Sdf).unwrap();

                    let width = glyph.bitmap().width();
                    let height = glyph.bitmap().rows();

                    let mut bytes = Vec::new();
                    bytes.write(glyph.bitmap().buffer()).unwrap();

                    match t_sender.send(CacheGlyph {
                        id: char,
                        bytes,
                        atlas_pos: Vec2::new(0.0, 0.0),
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
                // println!("finished {} {:?}", finished_count, st.elapsed());
            }
            if let Ok(recv) = receiver.try_recv() {
                if max_height < recv.height {
                    max_height = recv.height;
                }
                cache_glyphs.push(recv);
            }
            if finished_count == thread_count {
                break;
            }
        }
        println!("rendered in {:?}", st.elapsed());
        let st = Instant::now();

        let mut meta_data: Vec<u8> = Vec::new();
        println!("CAH {}", cache_glyphs.len());
        cache_glyphs.sort_by(|v1, v2| v1.height.cmp(&v2.height));

        // Creates the single texture atlas with all glyphs,
        // since swapping textures for every character is slow.
        // Is also in a single row to waste less pixel space
        let mut atlas_bytes: Vec<u8> = Vec::new();
        let mut atlas_width = 0;
        let mut atlas_height = 0;
        let mut index_offset = 0;
        loop {
            let mut end_index = index_offset;
            let mut max_height = 0;
            let mut row = 0;
            while row < max_height.max(1) {
                let mut x = 0;
                let mut y = atlas_height;
                for i in index_offset..cache_glyphs.len() {
                    let c_glyph = &mut cache_glyphs[i];
                    let offset = row * c_glyph.width;
                    if x + c_glyph.width > MAX_ATLAS_WIDTH || (atlas_width > 0 && x + c_glyph.width > atlas_width) {
                        if atlas_width == 0 {
                            atlas_width = x;
                        }
                        end_index = i;
                        for x in x..atlas_width {
                            atlas_bytes.push(0u8);
                        }
                        break;
                    }
                    if row == 0 {
                        c_glyph.atlas_pos = Vec2::new(x as f32, y as f32);
                    }

                    if c_glyph.height <= row {
                        for _ in 0..c_glyph.width { atlas_bytes.push(0u8); }
                    } else {
                        atlas_bytes.write(&c_glyph.bytes[offset as usize..(offset + c_glyph.width) as usize]).unwrap();
                    }

                    max_height = max_height.max(c_glyph.height);
                    x += c_glyph.width;
                }
                row += 1;
            }

            if index_offset == end_index {
                break;
            }

            atlas_height += max_height;
            index_offset = end_index;
        }
        println!("atlas in {:?}", st.elapsed());
        let st = Instant::now();

        // println!("{} {} {} {}", atlas_width, atlas_height, atlas_width*atlas_height, atlas_bytes.len());

        meta_data.write(&(size_metrics.ascender as f32 / 64.0).to_be_bytes()).unwrap();
        meta_data.write(&(size_metrics.descender as f32 / 64.0).to_be_bytes()).unwrap();
        meta_data.write(&cache_glyphs.len().to_be_bytes()).unwrap();
        for c_glyph in cache_glyphs.as_slice() {
            meta_data.write(&c_glyph.id.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.width.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.height.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.advance.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.bearing_x.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.top.to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.atlas_pos.x().to_be_bytes()).unwrap();
            meta_data.write(&c_glyph.atlas_pos.y().to_be_bytes()).unwrap();
        }

        meta_data.write(&atlas_width.to_be_bytes()).unwrap();
        meta_data.write(&atlas_height.to_be_bytes()).unwrap();
        meta_data.write_all(atlas_bytes.as_slice()).unwrap();
        println!("wrote in {:?}", st.elapsed());
        let st = Instant::now();
        BindTexture(TEXTURE_2D, 0);
        meta_data
    }

    /// A shortcut to calling
    ///
    /// ```
    /// use RustUI::components::render::font::manager::FontManager;
    /// FontManager::load(std::fs::read("cached_path").unwrap(), /*args_here*/);
    /// ```
    pub unsafe fn load_from_file(cached_path: &str) -> Self {
        let all_bytes = std::fs::read(cached_path).unwrap();
        Self::load(all_bytes)
    }

    /// Loads each char / glyph from the same format created by [Font::create_font_data]
    pub unsafe fn load(mut all_bytes: Vec<u8>) -> Self {
        let st = Instant::now();

        let mut index = 0;
        let ascent = f32::from_be_bytes(all_bytes[index..index+4].try_into().unwrap());
        index += 4;
        let decent = f32::from_be_bytes(all_bytes[index..index+4].try_into().unwrap());
        index += 4;
        let glyph_count = usize::from_be_bytes(all_bytes[index..index+8].try_into().unwrap());
        index += 8;

        let mut font = Font {
            glyphs: HashMap::new(),
            metrics: FontMetrics {
                ascent,
                decent,
            },
            atlas_tex: None,

            draw_data: Default::default(),
        };

        let mut atlas_height = 0f32;
        let mut atlas_width = 0f32;

        let mut v = 0;
        GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut v);
        PixelStorei(UNPACK_ALIGNMENT, 1);
        for i in 0..glyph_count {
            let id = usize::from_be_bytes(all_bytes[index..index+8].try_into().unwrap());
            index += 8;

            let width = i32_from_bytes(index, &all_bytes) as f32;
            index += 4;

            let height = i32_from_bytes(index, &all_bytes) as f32;
            index += 4;

            let advance = i32_from_bytes(index, &all_bytes) as f32;
            index += 4;

            let bearing_x = i32_from_bytes(index, &all_bytes) as f32;
            index += 4;

            let top = i32_from_bytes(index, &all_bytes) as f32;
            index += 4;

            let atlas_x = f32::from_be_bytes(all_bytes[index..index+4].try_into().unwrap());
            index += 4;

            let atlas_y = f32::from_be_bytes(all_bytes[index..index+4].try_into().unwrap());
            index += 4;

            if atlas_height < height {
                atlas_height = height;
            }

            font.glyphs.insert(
                id,
                Glyph {
                    atlas_pos: Vec2::new(atlas_x, atlas_y),
                    width,
                    height,
                    advance,
                    bearing_x,
                    top,
                }
            );

            atlas_width += width;
        }

        let width = i32_from_bytes(index, &all_bytes) as f32;
        index += 4;
        let height = i32_from_bytes(index, &all_bytes) as f32;
        index += 4;
        let atlas_bytes: &Vec<u8> = &all_bytes[index..].try_into().unwrap();

        let atlas_tex = Texture::create(width as i32, height as i32, atlas_bytes, ALPHA);
        font.atlas_tex = Some(atlas_tex);
        BindTexture(TEXTURE_2D, 0);

        font
    }


    /// Get (or create if it doesn't exist) the data for the text render batch
    pub unsafe fn get_inst(&mut self, formatted_text: impl Into<Text>, pos: impl Into<Vec2>, offset: impl Into<Vec2>) -> (u32, FontRenderData) {
        let offset = offset.into();
        let pos = pos.into();
        let formatted_text = formatted_text.into();
        let len = formatted_text.visible_length();
        let mut hasher = hash::DefaultHasher::new();
        offset.hash(&mut hasher);
        pos.hash(&mut hasher);

        formatted_text.hash(&mut hasher);

        let hashed = hasher.finish();

        self.draw_data = RenderData::default();

        let map = &mut context().fonts().cached_inst;
        if !map.contains_key(&hashed) {
            let (x, y) = pos.xy();

            self.draw_data.start_x = x;
            self.draw_data.line_width = offset.x;
            self.draw_data.x = x + offset.x;
            self.draw_data.y = y + offset.y;

            let mut current_color = Color::from_u32(0);

            self.draw_data.scale = 1.0;
            self.draw_data.i_scale = 1.0;
            self.draw_data.comb_scale_x = 1.0;
            self.draw_data.comb_scale_y = 1.0;

            let mut dims: Vec<[f32; 4]> = Vec::with_capacity(len);
            let mut uvs: Vec<[f32; 4]> = Vec::with_capacity(len);
            let mut colors: Vec<[u32; 4]> = Vec::with_capacity(len);
            let mut render_index = 0;
            let mut line_offsets = vec![];
            let mut line_ranges = vec![];
            let mut line_start_index = 0;

            let (a_width, a_height) = {
                let atlas = self.atlas_tex.as_ref().unwrap();
                (atlas.width as f32, atlas.height as f32)
            };

            let mut max_line_height = 0f32;
            let mut height = 0f32;
            let mut bounds = Vec4::xywh(self.draw_data.x, self.draw_data.y, 0.0, 0.0);

            for item in formatted_text.items() {
                match item {
                    FormatItem::None => {}
                    FormatItem::Color(v) => current_color = v.clone(),
                    FormatItem::AlignH(alignment) => {
                        if self.draw_data.current_align_h != 0. {
                            line_offsets.push( self.draw_data.line_width * self.draw_data.current_align_h);
                            line_ranges.push((line_start_index, render_index));
                            self.draw_data.line_width = 0.0;
                            self.draw_data.x = self.draw_data.start_x;
                        }
                        self.draw_data.current_align_h = alignment.get_value();
                        line_start_index = render_index;
                    },
                    FormatItem::AlignV(alignment) => {
                        self.draw_data.current_align_v = alignment.get_value();
                        line_start_index = render_index;
                    },
                    FormatItem::TabLength(v) => self.draw_data.current_tab_length = *v,
                    FormatItem::LineSpacing(v) => self.draw_data.current_line_spacing = *v,
                    FormatItem::Size(size) => {
                        let matrix: [f64; 16] = context().renderer().get_transform_matrix();
                        let scaled_factor_x = (matrix[0]*context().window().width as f64/2.0) as f32;
                        let scaled_factor_y = (matrix[5]*context().window().height as f64/-2.0) as f32;

                        self.draw_data.scale = size / FONT_RES as f32 * scaled_factor_x;
                        self.draw_data.i_scale = 1.0/ self.draw_data.scale;

                        self.draw_data.comb_scale_x = scaled_factor_x * self.draw_data.scale;
                        self.draw_data.comb_scale_y = scaled_factor_y * self.draw_data.scale;

                        max_line_height = max_line_height.max(self.get_line_height() * self.draw_data.scale);
                    }
                    FormatItem::Offset(amount) => {
                        self.draw_data.x += amount.x;
                        self.draw_data.y += amount.y;
                    }
                    FormatItem::String(string) => {
                        for char in string.chars() {
                            if char == '\n' {
                                if self.draw_data.current_align_h != 0. {
                                    line_offsets.push( self.draw_data.line_width * self.draw_data.current_align_h);
                                    line_ranges.push((line_start_index, render_index));
                                    // TODO track line heights
                                }

                                self.draw_data.y += max_line_height;
                                height += max_line_height;
                                max_line_height = 0f32;

                                line_start_index = render_index;
                                self.draw_data.line_width = 0.0;
                                self.draw_data.x = self.draw_data.start_x;

                                continue;
                            }

                            max_line_height = max_line_height.max(self.get_line_height() * self.draw_data.scale);

                            let (c_w, _c_h, c_a, should_render) = self.get_dimensions_scaled(char);
                            if should_render == 2 {
                                // println!("broken at {}", i);
                                // break
                            }

                            let glyph: &Glyph = match self.glyphs.get(&(char as usize)) {
                                None => continue,
                                Some(glyph) => glyph
                            };

                            let pos_y = self.draw_data.y + (self.get_height() - glyph.top) * self.draw_data.scale;

                            let (p_left, p_top, p_width, p_height) = (self.draw_data.x+glyph.bearing_x * self.draw_data.scale, pos_y, glyph.width * self.draw_data.scale, glyph.height * self.draw_data.scale);
                            let (uv_left, uv_top, uv_right, uv_bottom) = (glyph.atlas_pos.x / a_width, glyph.atlas_pos.y / a_height, (glyph.atlas_pos.x + glyph.width) / a_width, (glyph.atlas_pos.y + glyph.height) / a_height);

                            bounds.expand_to_x(p_left);
                            bounds.expand_to_y(p_top);
                            bounds.expand_to_x(p_left+p_width);
                            bounds.expand_to_y(p_top+p_height);

                            dims.push([p_left, p_top, p_width, p_height]);
                            uvs.push([uv_left, uv_top, uv_right-uv_left, uv_bottom-uv_top]);

                            // optimize to use u32 later

                            colors.push([current_color.rgba_u32(), 0x20ffffff, 0xfffffff0, 0xfffffff0]);

                            self.draw_data.x += c_a;
                            self.draw_data.line_width += c_a;
                            render_index += 1;
                        }
                    }
                }
            }
            if self.draw_data.current_align_h != 0. {
                line_offsets.push( self.draw_data.line_width * self.draw_data.current_align_h);
                line_ranges.push((line_start_index, render_index));
            }

            let mut line_index = 0;
            for (start, end) in &line_ranges {
                for i in *start..*end {
                    let mut dim = dims.get_mut(i).unwrap();
                    dim[0] -= line_offsets[line_index];
                }
                line_index += 1;
            }

            let shader = &context().fonts().sdf_shader;
            let mut vao = VertexArray::new();
            vao.bind();

            let mut dims_buf = Buffer::new(ARRAY_BUFFER);
            let (len, cap) = (dims.len(), dims.capacity());
            dims_buf.set_values(&dims);
            dims_buf.attribPointer(shader.get_attrib_location("dims") as GLuint, 4, FLOAT, FALSE, 1);

            let mut uvs_buf = Buffer::new(ARRAY_BUFFER);
            uvs_buf.set_values(&uvs);
            uvs_buf.attribPointer(shader.get_attrib_location("uvs") as GLuint, 4, FLOAT, FALSE, 1);

            let mut color = Buffer::new(ARRAY_BUFFER);
            color.set_values(&colors);
            color.attribIPointer(shader.get_attrib_location("colors") as GLuint, 4, UNSIGNED_INT, 1);

            let mut t_buf = Buffer::new(ARRAY_BUFFER);
            t_buf.set_values(&vec![0u8, 1u8, 2u8, 0u8, 2u8, 3u8]);
            t_buf.attribPointer(shader.get_attrib_location("ind") as GLuint, 1, UNSIGNED_BYTE, FALSE, 0);

            // Unbind VAO
            VertexArray::unbind();

            // Unbind buffers
            color.unbind();
            uvs_buf.unbind();

            // Add buffers to VAO object so they can be managed together
            vao.add_buffer(color);
            vao.add_buffer(uvs_buf);
            vao.add_buffer(t_buf);
            vao.add_buffer(dims_buf);

            // VertexArray, Vec2, Vec4, u32
            // (vao, Vec2::new(self.draw_data.line_width, height), bounds, 0)
            map.insert(hashed, (vao, 0, FontRenderData {
                end_char_pos: Vec2::new(self.draw_data.line_width, height),
                bounds,

                char_positions: Rc::new(dims),
                line_ranges: Rc::new(line_ranges),
            }));
        }
        map.get_mut(&hashed).unwrap().1 = 0;
        let res = map.get(&hashed).unwrap();
        (res.0.gl_ref(), res.2.clone())
    }

    /// Calls [`draw_string_offset()`] with an offset of `(0, 0)`
    ///
    /// [`draw_string_offset()`]: Font::draw_string_offset
    pub unsafe fn draw_string(&mut self, formatted_text: impl Into<Text>, pos: impl Into<Vec2>) -> FontRenderData {
        self.draw_string_offset(formatted_text, pos, (0, 0))
    }

    /// The method to be called to a render a string using modern GL
    ///
    /// Also caches the VAOs in order for faster rendering times,
    /// but is deleted if not used within 10 frames
    ///
    /// Returns width, height
    pub unsafe fn draw_string_offset(&mut self, formatted_text: impl Into<Text>, pos: impl Into<Vec2>, offset: impl Into<Vec2>) -> FontRenderData {
        let formatted_text = formatted_text.into();
        let pos = pos.into();

        let len = formatted_text.visible_length();

        context().renderer().stack().begin();
        context().renderer().stack().push(Blend(true));
        context().renderer().stack().push(Texture2D(true));
        self.bind_shader();
        // vec4.draw_vec4(0xffffffff);

        let atlas = self.atlas_tex.as_ref().unwrap();

        ActiveTexture(TEXTURE0);
        atlas.bind();

        let (vao, render_data) = self.get_inst(formatted_text, pos, offset);
        BindVertexArray(vao);
        // Finish();
        // let st = Instant::now();
        DrawArraysInstanced(TRIANGLES, 0, 6, len as GLsizei);
        // Finish();
        // println!("draw {} {:?}", len, st.elapsed());
        BindVertexArray(0);
        context().renderer().stack().end();

        Texture::unbind();
        self.end();
        render_data
        // (0f32, 0f32)
    }

    unsafe fn bind_shader(&self) {
        context().fonts().sdf_shader.bind();
        context().fonts().sdf_shader.u_put_float("u_res", vec![FONT_RES as f32]);
        let t = self.atlas_tex.as_ref().unwrap();
        let dims = vec![t.width as f32, t.height as f32];
        context().fonts().sdf_shader.u_put_float("u_atlas_dims", dims);
    }

    /// Returns the necessary dimensions of a glyph / character
    ///
    /// Returns `char_width, char_height, should_render`
    ///
    /// `should_render` is an integer that is 0, 1, or 2. Is calculated based off of this Font's current draw data
    /// ```
    /// use RustUI::components::render::font::Font;
    /// let should_render = unsafe { Font::get_dimensions_scaled('A') }.3;
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
    pub unsafe fn get_dimensions_scaled(&self, char: char) -> (f32, f32, f32, u32) {
        let glyph: &Glyph = match self.glyphs.get(&(char as usize)) {
            None => {
                return (0.0, 0.0, 0.0, 0);
            }
            Some(glyph) => {
                glyph
            }
        };

        let (c_w, c_h, c_a) = ((glyph.advance - glyph.bearing_x) * self.draw_data.scale, glyph.height as f32 * self.draw_data.scale, glyph.advance * self.draw_data.scale);
        let mut should_render = 0u32;
        if self.draw_data.y > context().window().width as f32 * self.draw_data.i_scale {
            should_render = 2;
        }
        else if self.draw_data.y > -c_h {
            should_render = 0;
        }
        else if self.draw_data.x <= context().window().height as f32 * self.draw_data.i_scale {
            should_render = 1;
        }
        (c_w, c_h, c_a, should_render)
    }

    pub unsafe fn end(&self) {
        // context().renderer().stack().pop();
        Shader::unbind();
        BindTexture(TEXTURE_2D, 0);
        Texture::unbind();
        Disable(BLEND);
    }

    /// Returns the width, in pixels, of a string at a specific size
    pub unsafe fn get_width(&self, size: f32, string: impl ToString) -> f32 {
        let string = string.to_string();
        let scale = size/FONT_RES as f32;
        let mut width = 0.0f32;

        for char in string.chars() {
            let glyph =  self.glyphs.get(&(char as usize)).unwrap();
            width += (glyph.advance - glyph.bearing_x) as f32;
        }

        width*scale
    }

    pub unsafe fn get_end_pos(&self, size: f32, string: impl ToString) -> Vec2 {
        let string = string.to_string();
        let scale = size/FONT_RES as f32;
        let mut width = 0f32;
        let mut height = 0f32;

        for char in string.chars() {
            let glyph =  self.glyphs.get(&(char as usize)).unwrap();
            if char == '\n' {
                width = 0f32;
                height += self.get_line_height();
                continue;
            }
            width += (glyph.advance - glyph.bearing_x) as f32;
        }

        (width*scale, height*scale).into()
    }

    pub unsafe fn add_end_pos(&self, current: Vec2, size: f32, string: impl ToString) -> Vec2 {
        let string = string.to_string();
        let scale = size/FONT_RES as f32;
        let mut width = current.x;
        let mut height = current.y;

        for char in string.chars() {
            let glyph = match self.glyphs.get(&(char as usize)) {
                None => continue,
                Some(v) => v
            };
            if char == '\n' {
                width = 0f32;
                height += self.get_line_height() * scale;
                continue;
            }
            width += (glyph.advance - glyph.bearing_x) * scale;
        }

        (width, height).into()
    }

    pub unsafe fn get_sized_height(&self, size: f32) -> f32 {
        let scale = size / FONT_RES as f32;
        self.get_height() * scale
    }

    /// Returns the height, in pixels, of the font. Unscaled
    pub unsafe fn get_height(&self) -> f32 {
        self.metrics.ascent + self.metrics.decent
        // self.glyphs.get('H' as usize).unwrap().top as f32 * scale
    }

    pub unsafe fn get_line_height(&self) -> f32 {
        self.get_height() * self.draw_data.current_line_spacing
    }
}

fn i32_from_bytes(index: usize, bytes: &Vec<u8>) -> i32 {
    i32::from_be_bytes(bytes[index..index+4].try_into().unwrap())
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Glyph {
    pub atlas_pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub advance: f32,
    pub bearing_x: f32,
    pub top: f32,
}
