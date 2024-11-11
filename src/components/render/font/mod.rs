pub mod format;
pub mod manager;
pub mod renderer;

extern crate freetype;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{hash, ptr};
use std::hash::{Hash, Hasher};
use std::sync::mpsc::channel;
use std::time::Instant;

use freetype::face::LoadFlag;
use freetype::RenderMode;
use gl::*;
use gl::types::{GLdouble, GLsizeiptr};

use crate::asset_manager;
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::position::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::renderer::Glyph;
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::{EnableClientState, Scalef, TexCoordPointer, TEXTURE_COORD_ARRAY, VertexPointer};
use crate::gl_binds::gl11::types::{GLsizei, GLuint};
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
    /// use RustUI::components::render::font::manager::FontManager;
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
