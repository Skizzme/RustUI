use std::fs::read_to_string;
use std::time::Instant;

use gl::*;
use gl::types::*;
use crate::components::render::bounds::Bounds;
use crate::components::render::shader::Shader;

use crate::gl_binds::gl30::{Begin, Color4d, End, PROJECTION_MATRIX, TexCoord2d, TexCoord2f, Vertex2d};

/// The global renderer to render basically everything non-text related
///
/// Uses mostly immediate GL, so best for a simple UI
#[derive(Debug)]
pub struct Renderer {
    rounded_rect_shader: Shader,
    pub texture_shader: Shader,
    pub(crate) color_mult_shader: Shader,
}

impl Renderer {

    pub unsafe fn new() -> Self {
        Renderer {
            rounded_rect_shader: Shader::new(
                read_to_string("src\\resources\\shaders\\rounded_rect\\vertex.glsl").unwrap(),
                read_to_string("src\\resources\\shaders\\rounded_rect\\fragment.glsl").unwrap()
            ),
            texture_shader: Shader::new(
                read_to_string("src\\resources\\shaders\\test_n\\vertex.glsl").unwrap(),
                read_to_string("src\\resources\\shaders\\test_n\\fragment.glsl").unwrap()
            ),
            color_mult_shader: Shader::new(
                read_to_string("src\\resources\\shaders\\color_mult\\vertex.glsl").unwrap(),
                read_to_string("src\\resources\\shaders\\color_mult\\fragment.glsl").unwrap()
            ),
        }
    }

    /// Should be called every frame, and whenever the matrix needs to be stored and sent to shaders
    pub unsafe fn get_transform_matrix(&self) -> [f64; 16] {
        let mut matrix: [f64; 16] = [0.0; 16];
        GetDoublev(PROJECTION_MATRIX, matrix.as_mut_ptr());
        matrix
    }

    /// Draws a nice rounded rectangle using texture shaders
    pub unsafe fn draw_rounded_rect(&self, bounds: &Bounds, radius: f32, color: u32) {
        Enable(BLEND);
        Enable(TEXTURE_2D);
        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.u_put_float("u_size", vec![bounds.width(), bounds.height()]);
        self.rounded_rect_shader.u_put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.u_put_float("u_color", self.get_rgb(color));

        self.draw_texture_rect(bounds, 0xffffffff);

        self.rounded_rect_shader.unbind();
        Disable(TEXTURE_2D);
        Disable(BLEND);
    }

    /// The most boring rectangle
    pub unsafe fn draw_rect(&self, bounds: &Bounds, color: u32) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        self.set_color(color);
        Begin(QUADS);
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        Vertex2d(bounds.right() as GLdouble, bounds.bottom() as GLdouble);
        Vertex2d(bounds.right() as GLdouble, bounds.top() as GLdouble);
        Vertex2d(bounds.left() as GLdouble, bounds.top() as GLdouble);
        End();
        Disable(BLEND);
    }

    /// A rectangle where each corner's color can be different
    ///
    /// Colors are in order of: bottom-left, bottom-right, top-right, top-left
    pub unsafe fn draw_gradient_rect(&self, bounds: &Bounds, color: (u32, u32, u32, u32)) {
        Enable(BLEND);
        Begin(QUADS);
        self.set_color(color.0);
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        self.set_color(color.1);
        Vertex2d(bounds.right() as GLdouble, bounds.bottom() as GLdouble);
        self.set_color(color.2);
        Vertex2d(bounds.right() as GLdouble, bounds.top() as GLdouble);
        self.set_color(color.3);
        Vertex2d(bounds.left() as GLdouble, bounds.top() as GLdouble);
        End();
        Disable(BLEND);
    }

    /// Draws only the outline of a rectangle
    pub unsafe fn draw_rect_outline(&self, bounds: &Bounds, width: f32, color: u32) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        self.set_color(color);
        LineWidth(width);
        Begin(LINE_STRIP);
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        Vertex2d(bounds.right() as GLdouble, bounds.bottom() as GLdouble);
        Vertex2d(bounds.right() as GLdouble, bounds.top() as GLdouble);
        Vertex2d(bounds.left() as GLdouble, bounds.top() as GLdouble);
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        End();
    }

    /// Draws a texture rectangle using normal UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect(&self, bounds: &Bounds, color: u32) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        Begin(QUADS);
        self.set_color(color);
        TexCoord2d(0.0, 1.0);
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        TexCoord2d(1.0, 1.0);
        Vertex2d(bounds.right() as GLdouble, bounds.bottom() as GLdouble);
        TexCoord2d(1.0, 0.0);
        Vertex2d(bounds.right() as GLdouble, bounds.top() as GLdouble);
        TexCoord2d(0.0, 0.0);
        Vertex2d(bounds.left() as GLdouble, bounds.top() as GLdouble);
        End();
    }

    /// Draws a texture rectangle using specified UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect_uv(&self, bounds: &Bounds, uv_bounds: &Bounds, color: u32) {
        Enable(TEXTURE_2D);
        Begin(QUADS);
        self.set_color(color);
        TexCoord2f(uv_bounds.left(), uv_bounds.bottom());
        Vertex2d(bounds.left() as GLdouble, bounds.bottom() as GLdouble);
        TexCoord2f(uv_bounds.right(), uv_bounds.bottom());
        Vertex2d(bounds.right() as GLdouble, bounds.bottom() as GLdouble);
        TexCoord2f(uv_bounds.right(), uv_bounds.top());
        Vertex2d(bounds.right() as GLdouble, bounds.top() as GLdouble);
        TexCoord2f(uv_bounds.left(), uv_bounds.top());
        Vertex2d(bounds.left() as GLdouble, bounds.top() as GLdouble);
        End();
    }

    /// Converts the `color` to RGBA and calls gl_Color4d
    pub unsafe fn set_color(&self, color: u32) {
        let alpha = (color >> 24 & 255) as f32 / 255f32;
        let red = (color >> 16 & 255) as f32 / 255f32;
        let green = (color >> 8 & 255) as f32 / 255f32;
        let blue = (color & 255) as f32 / 255f32;
        Color4d(red as GLdouble, green as GLdouble, blue as GLdouble, alpha as GLdouble);
    }

    /// Returns the RGBA of u32 color
    pub fn get_rgb(&self, color: u32) -> Vec<f32> {
        let alpha = (color >> 24 & 255) as f32 / 255f32;
        let red = (color >> 16 & 255) as f32 / 255f32;
        let green = (color >> 8 & 255) as f32 / 255f32;
        let blue = (color & 255) as f32 / 255f32;
        vec![red, green, blue, alpha]
    }
}