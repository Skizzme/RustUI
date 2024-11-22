extern crate freetype;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::io::Write;
use std::num::NonZero;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::channel;
use std::time::Instant;

use freetype::face::LoadFlag;
use freetype::RenderMode;
use gl::*;
use crate::components::position::Vec2;

use crate::components::render::font::renderer::Glyph;
use crate::components::wrapper::texture::Texture;

pub mod format;
pub mod manager;
pub mod renderer;

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

/// Only holds the data per font to be used by the font renderer
pub struct Font {
    pub glyphs: HashMap<usize, Glyph>,
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
                let lib = freetype::Library::init().unwrap();

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
}

fn i32_from_bytes(index: usize, bytes: &Vec<u8>) -> i32 {
    i32::from_be_bytes(bytes[index..index+4].try_into().unwrap())
}