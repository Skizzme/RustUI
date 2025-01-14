use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use crate::components::render::font::{Font, FONT_RES};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::buffer::VertexArray;
use crate::components::wrapper::shader::Shader;
use crate::test_ui::asset_manager;

/// An easy-to-use system for loading and getting fonts
pub struct FontManager {
    pub(crate) fonts: HashMap<String, Font>,
    cache_location: String,
    mem_atlas_cache: HashMap<String, Vec<u8>>,
    pub(crate) sdf_shader: Shader,
    font_byte_library: HashMap<String, Vec<u8>>,
    pub(crate) cached_inst: HashMap<u64, (VertexArray, Vec2, Vec4, u32)>,
}

impl FontManager {
    pub unsafe fn new(cache_location: impl ToString) -> Self {
        let st = Instant::now();
        let s = Shader::new(asset_manager::file_contents_str("shaders/sdf/vertex.glsl").unwrap(), asset_manager::file_contents_str("shaders/sdf/fragment.glsl").unwrap());
        FontManager {
            fonts: HashMap::new(),
            cache_location: cache_location.to_string(),
            mem_atlas_cache: HashMap::new(),
            sdf_shader: s,
            font_byte_library: HashMap::new(),
            cached_inst: HashMap::new(),
        }
    }

    pub unsafe fn cleanup(&mut self) {
        let mut remove = vec![];
        for (hash, (_, _, _, frames_elapsed)) in &mut self.cached_inst {
            if *frames_elapsed > 10 {
                remove.push(*hash);
            } else {
                *frames_elapsed += 1;
            }
        }

        remove.iter().for_each(|key| unsafe {
            let (vao, _, _, _) = self.cached_inst.remove(&key).unwrap();
            vao.delete();
        });
    }

    pub fn font(&mut self, name: impl ToString) -> Option<&mut Font> {
        let name = name.to_string();
        if !self.fonts.contains_key(&name) {
            unsafe {
                self.load_font(name.clone(), false);
            }
        }
        self.fonts.get_mut(&name)
    }

    /// Sets the byte-data for a font to be used by the loader so that fonts don't have to be files
    ///
    /// Should only be used on setup, and the memory will be freed once the font is loaded
    ///
    pub fn set_font_bytes(&mut self, name: impl ToString, font_bytes: Vec<u8>) -> &mut FontManager {
        self.font_byte_library.insert(name.to_string(), font_bytes);
        self
    }


    /// Loads a font using the same name specified when specifying font bytes using [`set_font_bytes()`]
    ///
    /// [`set_font_bytes()`]: FontManager::set_font_bytes
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
                println!("Font '{}' took {:?} to render and load ({})...", &name, b.elapsed(), ft.glyphs.len());

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
}