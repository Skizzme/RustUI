use std::hash;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use gl::{ActiveTexture, ARRAY_BUFFER, BindTexture, BindVertexArray, BLEND, Disable, FLOAT, TEXTURE0, TEXTURE_2D, TRIANGLES};

use crate::components::context::context;
use crate::components::position::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::{Font, FONT_RES};
use crate::components::render::font::format::{FormatItem, FormattedText};
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::{FALSE, Finish, Scaled, Scalef};
use crate::gl_binds::gl11::types::{GLsizei, GLuint};
use crate::gl_binds::gl30::{PopMatrix, PushMatrix};
use crate::gl_binds::gl41::DrawArraysInstanced;

/// The object used to render fonts
///
/// Contains options like tab length, line spacing, wrapping etc. for convenience
///
/// It would be preferable not to be created each frame
pub struct FontRenderer {
    pub font: String,
    pub wrapping: Wrapping,
    pub tab_length: u32, // The length of tabs in spaces. Default is 4
    pub line_spacing: f32,
    pub line_width: f32,
    pub alignment: (AlignmentH, AlignmentV),

    scale: f32,
    i_scale: f32,
    x: f32,
    y: f32,
    start_x: f32,
    comb_scale_x: f32,
    comb_scale_y: f32,
}

impl FontRenderer {
    pub unsafe fn new(font: String) -> Self {
        FontRenderer {
            font,
            tab_length: 4,
            line_spacing: 1.5,
            wrapping: Wrapping::None,
            line_width: 0.0,
            alignment: (AlignmentH::Left, AlignmentV::Top),
            scale: 0.0,
            i_scale: 0.0,
            x: 0.0,
            y: 0.0,
            start_x: 0.0,
            comb_scale_x: 0.0,
            comb_scale_y: 0.0,
        }
    }

    /// Renders a string using immediate GL
    ///
    /// The center of the rendered string is at `x`
    // pub unsafe fn draw_centered_string(&mut self, size: f32, string: impl ToString, x: f32, y: f32, color: impl ToColor) -> (f32, f32) {
        // let string = string.to_string();
        // let width = self.get_width(size, string.clone());
        // self.draw_string(size, string, (x-width/2.0, y), color)
        // (0.0, 0.0)
    // }

    unsafe fn get_or_cache_inst(&mut self, formatted_text: impl Into<FormattedText>, pos: impl Into<Vec2>) -> (u32, f32, f32) {
        let formatted_text = formatted_text.into();
        let len = formatted_text.visible_length();
        let mut hasher = hash::DefaultHasher::new();
        formatted_text.hash(&mut hasher);

        let hashed = hasher.finish();

        let map = &mut context().fonts().cached_inst;
        if !map.contains_key(&hashed) {
            let mut dims: Vec<[f32; 4]> = Vec::with_capacity(len);
            let mut uvs: Vec<[f32; 4]> = Vec::with_capacity(len);
            let mut colors: Vec<[f32; 4]> = Vec::with_capacity(len);
            let (x, y) = pos.into().xy();

            self.x = x;
            self.y = y;
            self.start_x = x;

            // Apply appropriate scale to the vertices etc for correct rendering

            let mut current_color = Color::from_u32(0);

            // Calculate vertices and uv coords for every char

            self.scale = 1.0;
            self.i_scale = 1.0;
            self.comb_scale_x = 1.0;
            self.comb_scale_y = 1.0;

            self.line_width = 0f32;

            for item in formatted_text.items() {
                match item {
                    FormatItem::None => {}
                    FormatItem::Color(v) => {
                        current_color = v.clone();
                    }
                    FormatItem::Size(size) => {
                        // Scalef(1.0 / self.scale, 1.0 / self.scale, 1.0); // unscale the current scaling

                        let matrix: [f64; 16] = context().renderer().get_transform_matrix();
                        let scaled_factor_x = (matrix[0]*context().window().width as f64/2.0) as f32;
                        let scaled_factor_y = (matrix[5]*context().window().height as f64/-2.0) as f32;

                        self.scale = size / FONT_RES as f32 *  scaled_factor_x;
                        self.i_scale = 1.0/ self.scale;

                        // self.x = x * self.i_scale;
                        // self.y = y * self.i_scale;

                        self.comb_scale_x = scaled_factor_x * self.scale;
                        self.comb_scale_y = scaled_factor_y * self.scale;

                        // Scaled(self.scale as f64, self.scale as f64, 1.0);
                    }
                    FormatItem::Offset(_) => {}
                    FormatItem::Text(string) => {
                        for char in string.chars() {
                            if char == '\n' {
                                self.y += self.get_line_height() * self.scale;
                                self.line_width = 0.0;
                                self.x = self.start_x;
                                continue;
                            }

                            let (c_w, _c_h, _, _) = self.get_dimensions(char);

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
                            let pos_y = self.y + (self.get_height() - glyph.top as f32) * self.scale;

                            let (right, bottom) = (self.x+glyph.width as f32 * self.scale, pos_y+glyph.height as f32 * self.scale);

                            let (p_left, p_top, p_right, p_bottom) = (self.x+glyph.bearing_x as f32 * self.scale, pos_y, right, bottom);
                            let atlas = self.font().atlas_tex.as_ref().unwrap();
                            let (uv_left, uv_top, uv_right, uv_bottom) = (glyph.atlas_x as f32 / atlas.width as f32, 0f32, (glyph.atlas_x + glyph.width) as f32 / atlas.width as f32, glyph.height as f32 / atlas.height as f32);

                            dims.push([p_left, p_top, p_right-p_left, p_bottom-p_top]);
                            uvs.push([uv_left, uv_top, uv_right-uv_left, uv_bottom-uv_top]);
                            // optimize to use u32 later
                            colors.push(current_color.rgba());

                            self.x += c_w;
                            self.line_width += c_w;
                        }
                    }
                }
            }

            let shader = &context().fonts().sdf_shader;
            let mut vao = VertexArray::new();
            vao.bind();

            let mut dims_buf = Buffer::new(ARRAY_BUFFER);
            let (len, cap) = (dims.len(), dims.capacity());
            dims_buf.set_values(dims);
            dims_buf.attribPointer(shader.get_attrib_location("dims") as GLuint, 4, FLOAT, FALSE, 1);

            let mut uvs_buf = Buffer::new(ARRAY_BUFFER);
            uvs_buf.set_values(uvs);
            uvs_buf.attribPointer(shader.get_attrib_location("uvs") as GLuint, 4, FLOAT, FALSE, 1);

            let mut color = Buffer::new(ARRAY_BUFFER);
            color.set_values(colors);
            color.attribPointer(shader.get_attrib_location("color") as GLuint, 4, FLOAT, FALSE, 1);

            let mut t_buf = Buffer::new(ARRAY_BUFFER);
            t_buf.set_values(vec![0f32, 1f32, 2f32, 0f32, 2f32, 3f32]);
            t_buf.attribPointer(shader.get_attrib_location("ind") as GLuint, 1, FLOAT, FALSE, 0);

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

            map.insert(hashed, (vao, self.line_width, self.get_line_height()*self.scale, 0));
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
    pub unsafe fn draw_string(&mut self, formatted_text: impl Into<FormattedText>, pos: impl Into<Vec2>) -> (f32, f32) {
        let formatted_text = formatted_text.into();
        let pos = pos.into();

        let len = formatted_text.visible_length();

        let (vao, width, height) = self.get_or_cache_inst(formatted_text, pos);
        context().renderer().stack().begin();
        context().renderer().stack().push(Blend(true));
        context().renderer().stack().push(Texture2D(true));
        self.bind_shader();
        let atlas = self.font().atlas_tex.as_ref().unwrap();

        ActiveTexture(TEXTURE0);
        atlas.bind();

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
        (width, height)
        // (0f32, 0f32)
    }

    unsafe fn bind_shader(&self) {
        context().fonts().sdf_shader.bind();
        context().fonts().sdf_shader.u_put_float("u_res", vec![FONT_RES as f32]);
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

        let (c_w, c_h, c_a) = ((glyph.advance - glyph.bearing_x) as f32 * self.scale, glyph.height as f32 * self.scale, glyph.advance as f32 * self.scale);
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
        self.get_height() * (self.line_spacing)
    }

    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    pub fn alignment(mut self, alignment: (AlignmentH, AlignmentV)) -> Self {
        self.alignment = alignment;
        self
    }

    unsafe fn font(&self) -> &Font {
        context().fonts().fonts.get(&self.font).unwrap()
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Glyph {
    pub atlas_x: i32,
    pub width: i32,
    pub height: i32,
    pub advance: i32,
    pub bearing_x: i32,
    pub top: i32,
}

pub enum AlignmentH {
    Left, // 0.0
    Middle, // 0.5
    Right, // 1.0
    Custom(f32)
}

pub enum AlignmentV {
    Top, // 0.0
    Middle, // 0.5
    Bottom, // 1.0
    Custom(f32),
}