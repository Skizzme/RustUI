use std::{hash, ptr};
use std::hash::{Hash, Hasher};
use gl::{ActiveTexture, ARRAY_BUFFER, BindTexture, BindVertexArray, BLEND, Disable, DrawElements, ELEMENT_ARRAY_BUFFER, FLOAT, TEXTURE0, TEXTURE_2D, TRIANGLES, UNSIGNED_INT, VERTEX_ARRAY};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::position::Pos;
use crate::components::render::color::ToColor;
use crate::components::render::font::{Font, FONT_RES};
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::{EnableClientState, Scalef, VertexPointer};
use crate::gl_binds::gl11::types::GLsizei;
use crate::gl_binds::gl30::{PopMatrix, PushMatrix};

/// The object used to render fonts
///
/// Contains options like tab length, line spacing, wrapping etc. for convenience
///
/// It would be preferable not to be created each frame
pub struct FontRenderer {
    pub font: String,
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

impl FontRenderer {
    pub unsafe fn new(font: String) -> Self {
        FontRenderer {
            font,
            tab_length: 4,
            line_spacing: 20.0,
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
        self.draw_string(size, string, (x-width/2.0, y), color)
    }

    unsafe fn get_or_cache_inst(&mut self, size: f32, string: String, pos: Pos) -> (u32, f32, f32) {
        let mut hasher = hash::DefaultHasher::new();
        hasher.write(&size.to_be_bytes());
        hasher.write(string.as_bytes());
        pos.hash(&mut hasher);

        let hashed = hasher.finish();
        let mut map = &mut context().fonts().cached_inst;
        if !map.contains_key(&hashed) {
            let mut vertices: Vec<[f32; 4]> = Vec::with_capacity(string.len());
            let mut indices: Vec<u32> = Vec::with_capacity(string.len());
            let (x, y) = pos.xy();

            // Apply appropriate scale to the vertices etc for correct rendering
            self.begin(size, x, y, true);
            // println!("{} {} {}", self.get_line_height(), self.comb_scale_y, self.line_spacing);
            // Calculate vertices and uv coords for every char
            for char in string.chars() {
                if char == '\n' {
                    match self.scale_mode {
                        ScaleMode::Normal => {
                            self.y += self.get_line_height();
                        }
                        ScaleMode::Quality => {
                            self.y += self.get_scaled_value(self.get_line_height(), self.comb_scale_y);
                        }
                    }
                    self.line_width = 0.0;
                    self.x = self.start_x;
                    continue;
                }

                let (c_w, _c_h, c_a, should_render) = self.get_dimensions(char);

                let glyph: &Glyph = match self.font().glyphs.get(char as usize) {
                    None => {
                        continue
                        // return 0;
                        // return (0.0, 0.0);
                    }
                    Some(glyph) => {
                        glyph
                    }
                };
                let pos_y = self.y + self.get_height() - glyph.top as f32;

                let (right, bottom) = match self.scale_mode {
                    ScaleMode::Normal => {
                        (self.x+glyph.width as f32, pos_y+glyph.height as f32)
                    }
                    ScaleMode::Quality => {
                        (self.get_scaled_value(self.x+glyph.width as f32, self.comb_scale_x), self.get_scaled_value(pos_y+glyph.height as f32, self.comb_scale_y))
                    }
                };
                let (p_left, p_top, p_right, p_bottom) = (self.x+glyph.bearing_x as f32, pos_y, right, bottom);
                let atlas = self.font().atlas_tex.as_ref().unwrap();
                let (uv_left, uv_top, uv_right, uv_bottom) = (glyph.atlas_x as f32 / atlas.width as f32, 0f32, (glyph.atlas_x + glyph.width) as f32 / atlas.width as f32, glyph.height as f32 / atlas.height as f32);

                vertices.push([p_left, p_bottom, uv_left, uv_bottom]);
                vertices.push([p_right, p_bottom, uv_right, uv_bottom]);
                vertices.push([p_right, p_top, uv_right, uv_top]);
                vertices.push([p_left, p_top, uv_left, uv_top]);

                let base = vertices.len() as u32 - 4;

                indices.push(base+0);
                indices.push(base+1);
                indices.push(base+2);
                indices.push(base+0);
                indices.push(base+2);
                indices.push(base+3);

                self.x += c_w;
                self.line_width += c_w;
            }
            self.end();

            let mut vao = VertexArray::new();
            vao.bind();

            let mut vert = Buffer::new(ARRAY_BUFFER);
            vert.set_values(vertices);

            let mut element_buf = Buffer::new(ELEMENT_ARRAY_BUFFER);
            element_buf.set_values(indices);

            // Bind buffers into the VAO
            vert.bind();
            EnableClientState(VERTEX_ARRAY);
            VertexPointer(4, FLOAT, 0, ptr::null());

            element_buf.bind();

            // Unbind VAO
            VertexArray::unbind();

            // Unbind buffers
            element_buf.unbind();
            vert.unbind();

            // Add buffers to VAO object so they can be managed together
            vao.add_buffer(vert);
            vao.add_buffer(element_buf);

            map.insert(hashed, (vao, self.line_width*self.scale, self.get_line_height()*self.scale, 0));
        }
        map.get_mut(&hashed).unwrap().3 = 0;
        let (vao, width, height, _) = map.get(&hashed).unwrap();
        (vao.gl_ref(), *width, *height)
    }

    /// The method to be called to a render a string using modern GL
    ///
    /// Also caches the VAOs in order to be even more effective,
    /// but is deleted if not used within 10 frames
    ///
    /// Returns width, height
    pub unsafe fn draw_string_inst(&mut self, size: f32, string: impl ToString, pos: impl Into<Pos>, color: impl ToColor) -> (f32, f32) {
        let string = string.to_string();
        let pos = pos.into();

        let (x, y) = pos.xy();
        let len = string.len();

        let (vao, width, height) = self.get_or_cache_inst(size, string, pos);
        self.begin(size, x, y, true);
        self.set_color(color);
        let atlas = self.font().atlas_tex.as_ref().unwrap();

        ActiveTexture(TEXTURE0);
        atlas.bind();

        BindVertexArray(vao);
        DrawElements(TRIANGLES, (len * 6) as GLsizei, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);

        Texture::unbind();
        self.end();
        (width, height)
    }

    /// The method to be called to a render a string using immediate GL
    ///
    /// Returns width, height
    pub unsafe fn draw_string(&mut self, size: f32, string: impl ToString, pos: impl Into<Pos>, color: impl ToColor) -> (f32, f32) {
        // let str_height = self.font().glyphs.get('H' as usize).unwrap().top as f32;
        let (x, y) = pos.into().xy();
        self.begin(size, x, y, false);
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

            let (c_w, _c_h, c_a, should_render) = self.get_dimensions(char);
            // if should_render == 2 {
            //     break;
            // }

            // if should_render <= 1 {
            //     if should_render == 0 {
            let atlas_ref= self.font().atlas_tex.as_ref().unwrap().clone();
            self.draw_char(self.comb_scale_x, self.comb_scale_y, &atlas_ref, char, self.x, self.y);
            // }

            self.line_width += c_w;
            match self.scale_mode {
                ScaleMode::Normal => {
                    self.x += c_a;
                }
                ScaleMode::Quality => {
                    self.x += self.get_scaled_value(c_a, self.comb_scale_x);
                }
            }
            // }
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
    /// use RustUI::components::render::font::renderer::FontRenderer;
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
    pub unsafe fn get_dimensions(&self, char: char) -> (f32, f32, f32, u32) {
        let glyph: &Glyph = match self.font().glyphs.get(char as usize) {
            None => {
                return (0.0, 0.0, 0.0, 0);
            }
            Some(glyph) => {
                glyph
            }
        };

        let (c_w, c_h, c_a) = match self.scale_mode {
            ScaleMode::Normal => ((glyph.advance - glyph.bearing_x) as f32, glyph.height as f32, glyph.advance as f32),
            ScaleMode::Quality => (((glyph.advance - glyph.bearing_x) as f32).ceil(), (glyph.height as f32).ceil(), (glyph.advance as f32).ceil())
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
        (c_w, c_h, c_a, should_render)
    }

    /// Draws a single char
    ///
    /// The exact draw methods are determined by this FontRenderer's options, like [FontRenderer::scale_mode] etc
    pub unsafe fn draw_char(&mut self, scaled_x: f32, scaled_y: f32, atlas: &Texture, char: char, x: f32, y: f32) -> (f32, f32) {
        let glyph: &Glyph = match self.font().glyphs.get(char as usize) {
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
        let uv = Bounds::ltrb(glyph.atlas_x as f32 / atlas.width as f32, 0f32, (glyph.atlas_x + glyph.width) as f32 / atlas.width as f32, glyph.height as f32 / atlas.height as f32);
        // println!("N: {} {:?}", char, uv);
        context().renderer().draw_texture_rect_uv(
            &Bounds::ltrb(x+glyph.bearing_x as f32, pos_y, right, bottom),
            &uv,
            0xffffff,
        );
        // TODO make rendering use bearing x correctly
        match self.scale_mode {
            ScaleMode::Normal => (((glyph.advance - glyph.bearing_x) as f32), (glyph.height as f32)),
            ScaleMode::Quality => (((glyph.advance - glyph.bearing_x) as f32).floor(), (glyph.height as f32).floor())
        }
    }

    /// Sets this FontRenderer up for immediate GL drawing, setting shader uniforms, x and y offsets, scaling etc
    pub unsafe fn begin(&mut self, size: f32, x: f32, y: f32, instanced_shader: bool) {
        let matrix: [f64; 16] = context().renderer().get_transform_matrix();
        self.scaled_factor_x = (matrix[0]*context().window().width as f64/2.0) as f32;
        self.scaled_factor_y = (matrix[5]*context().window().height as f64/-2.0) as f32;

        self.scale = match self.scale_mode {
            ScaleMode::Normal => {size/FONT_RES as f32 * self.scaled_factor_x}
            ScaleMode::Quality => {size.ceil()/FONT_RES as f32 * self.scaled_factor_x}
        };
        self.i_scale = 1.0/self.scale;

        context().renderer().stack().begin();
        context().renderer().stack().push(Blend(true));
        context().renderer().stack().push(Texture2D(true));
        self.x = x*self.i_scale;
        self.y = y*self.i_scale;
        self.start_x = self.x;

        self.comb_scale_x = self.scaled_factor_x*self.scale;
        self.comb_scale_y = self.scaled_factor_y*self.scale;

        // println!("fr {} {} {}", self.scaled_factor_x, self.comb_scale_x, self.scale);
        PushMatrix();
        Scalef(self.scale, self.scale, 1.0);

        self.line_width = 0f32;

        let atlas = self.font().atlas_tex.as_ref().unwrap();
        atlas.bind();
        // was 0.25 / ... but .35 seems better?
        //(0.30 / (size / 9.0 *self.scaled_factor_x.max(self.scaled_factor_y)) * FONT_RES as f32 / 64.0).clamp(0.0, 0.4) // original smoothing
        let smoothing = (0.35 / (size / 6.0 *self.scaled_factor_x.max(self.scaled_factor_y)) * FONT_RES as f32 / 64.0).clamp(0.0, 0.25);

        let shader =
            if instanced_shader {
                &mut context().fonts().sdf_shader_i
            } else {
                &mut context().fonts().sdf_shader
            };

        shader.bind();
        shader.u_put_float("u_smoothing", vec![smoothing]);
        shader.u_put_float("atlas_width", vec![atlas.width as f32]);
        shader.u_put_float("i_scale", vec![1.0/self.comb_scale_x]);

    }

    pub unsafe fn end(&self) {
        // context().renderer().stack().pop();
        context().renderer().stack().end();
        Shader::unbind();
        BindTexture(TEXTURE_2D, 0);
        PopMatrix();
        Texture::unbind();
        Disable(BLEND);
    }

    /// Returns the width, in pixels, of a string at a specific size
    pub unsafe fn get_width(&self, size: f32, string: impl ToString) -> f32 {
        let string = string.to_string();
        let scale = size/FONT_RES as f32;
        let mut width = 0.0f32;

        for char in string.chars() {
            let glyph =  self.font().glyphs.get(char as usize).unwrap();
            width += (glyph.advance - glyph.bearing_x) as f32;
        }

        width*scale
    }

    /// Returns the height, in pixels, of the font. Unscaled
    pub unsafe fn get_height(&self) -> f32 {
        self.font().metrics.ascent + self.font().metrics.decent
        // self.font().glyphs.get('H' as usize).unwrap().top as f32 * scale
    }

    pub unsafe fn get_line_height(&self) -> f32 {
        self.get_height() + (self.line_spacing)
    }

    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    unsafe fn font(&self) -> &Font {
        context().fonts().fonts.get(&self.font).unwrap()
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