use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use crate::asset_manager;
use crate::components::render::font::{Font, FONT_RES};
use crate::components::render::font::renderer::FontRenderer;
use crate::components::wrapper::buffer::VertexArray;
use crate::components::wrapper::shader::Shader;

pub struct FontManager {
    pub(crate) fonts: HashMap<String, Font>,
    cache_location: String,
    mem_atlas_cache: HashMap<String, Vec<u8>>,
    pub(crate) sdf_shader: Shader,
    pub(crate) sdf_shader_i: Shader,
    font_byte_library: HashMap<String, Vec<u8>>,
    pub cached_inst: HashMap<u64, (VertexArray, f32, f32, u32)>,
}

impl FontManager {
    pub unsafe fn new(cache_location: impl ToString) -> Self {
        let st = Instant::now();
        let s = Shader::new(asset_manager::file_contents_str("shaders/sdf/vertex.glsl").unwrap(), asset_manager::file_contents_str("shaders/sdf/fragment.glsl").unwrap());
        let s_instanced = Shader::new(asset_manager::file_contents_str("shaders/sdf_instanced/vertex.glsl").unwrap(), asset_manager::file_contents_str("shaders/sdf_instanced/fragment.glsl").unwrap());
        println!("{}", st.elapsed().as_secs_f32());
        FontManager {
            fonts: HashMap::new(),
            cache_location: cache_location.to_string(),
            mem_atlas_cache: HashMap::new(),
            sdf_shader: s,
            sdf_shader_i: s_instanced,
            font_byte_library: HashMap::new(),
            cached_inst: HashMap::new(),
        }
    }

    pub unsafe fn cleanup(&mut self) {
        let mut remove = vec![];
        for (hash, (vao, width, height, frames_elapsed)) in &mut self.cached_inst {
            if *frames_elapsed > 10 {
                remove.push(*hash);
            } else {
                *frames_elapsed += 1;
            }
        }
        println!("CACHED {} REMOVE {}", self.cached_inst.len(), remove.len());
        remove.iter().for_each(|key| unsafe {
            let (vao, width, height, _) = self.cached_inst.remove(&key).unwrap();
            vao.delete();
        });
    }

    pub fn font(&self, name: impl ToString) -> Option<&Font> {
        self.fonts.get(&name.to_string())
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
    pub unsafe fn renderer(&mut self, name: &str) -> FontRenderer {
        if !self.fonts.contains_key(name) {
            self.load_font(name, false);
        }
        FontRenderer::new(name.to_string())
    }
}