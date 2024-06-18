use gl::*;

use crate::asset_manager;
use crate::components::render::bounds::Bounds;
use crate::components::render::color::ToColor;
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl30::{Begin, End, PROJECTION_MATRIX, TexCoord2d, TexCoord2f, Vertex2f};

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
                asset_manager::file_contents_str("shaders\\rounded_rect\\vertex.glsl").expect("Failed to read shader file"),
                asset_manager::file_contents_str("shaders\\rounded_rect\\fragment.glsl").expect("Failed to read shader file"),
            ),
            texture_shader: Shader::new(
                asset_manager::file_contents_str("shaders\\test_n\\vertex.glsl").expect("Failed to read shader file"),
                asset_manager::file_contents_str("shaders\\test_n\\fragment.glsl").expect("Failed to read shader file"),
            ),
            color_mult_shader: Shader::new(
                asset_manager::file_contents_str("shaders\\color_mult\\vertex.glsl").expect("Failed to read shader file"),
                asset_manager::file_contents_str("shaders\\color_mult\\fragment.glsl").expect("Failed to read shader file"),
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
    pub unsafe fn draw_rounded_rect(&self, bounds: &Bounds, radius: f32, color: impl ToColor) {
        Enable(BLEND);
        Enable(TEXTURE_2D);
        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.u_put_float("u_size", vec![bounds.width(), bounds.height()]);
        self.rounded_rect_shader.u_put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.u_put_float("u_color", color.to_color().rgba());

        self.draw_texture_rect(bounds, 0xffffffff);

        self.rounded_rect_shader.unbind();
        Disable(TEXTURE_2D);
        Disable(BLEND);
    }

    /// The most boring rectangle
    pub unsafe fn draw_rect(&self, bounds: &Bounds, color: impl ToColor) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        color.apply_color();
        Begin(QUADS);
        Vertex2f(bounds.left(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.top());
        Vertex2f(bounds.left(), bounds.top());
        End();
        Disable(BLEND);
    }

    /// A rectangle where each corner's color can be different
    ///
    /// Colors are in order of: bottom-left, bottom-right, top-right, top-left
    pub unsafe fn draw_gradient_rect(&self, bounds: &Bounds, color: (impl ToColor, impl ToColor, impl ToColor, impl ToColor)) {
        Enable(BLEND);
        Begin(QUADS);
        color.0.apply_color();
        Vertex2f(bounds.left(), bounds.bottom());
        color.1.apply_color();
        Vertex2f(bounds.right(), bounds.bottom());
        color.2.apply_color();
        Vertex2f(bounds.right(), bounds.top());
        color.3.apply_color();
        Vertex2f(bounds.left(), bounds.top());
        End();
        Disable(BLEND);
    }

    /// Draws only the outline of a rectangle
    pub unsafe fn draw_rect_outline(&self, bounds: &Bounds, width: f32, color: impl ToColor) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        color.apply_color();
        LineWidth(width);
        Begin(LINE_STRIP);
        Vertex2f(bounds.left(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.top());
        Vertex2f(bounds.left(), bounds.top());
        Vertex2f(bounds.left(), bounds.bottom());
        End();
    }

    /// Draws a texture rectangle using normal UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect(&self, bounds: &Bounds, color: impl ToColor) {
        Disable(TEXTURE_2D);
        Enable(BLEND);
        Begin(QUADS);
        color.apply_color();
        TexCoord2d(0.0, 1.0);
        Vertex2f(bounds.left(), bounds.bottom());
        TexCoord2d(1.0, 1.0);
        Vertex2f(bounds.right(), bounds.bottom());
        TexCoord2d(1.0, 0.0);
        Vertex2f(bounds.right(), bounds.top());
        TexCoord2d(0.0, 0.0);
        Vertex2f(bounds.left(), bounds.top());
        End();
    }

    /// Draws a texture rectangle using specified UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect_uv(&self, bounds: &Bounds, uv_bounds: &Bounds, color: impl ToColor) {
        Enable(TEXTURE_2D);
        Begin(QUADS);
        color.apply_color();
        TexCoord2f(uv_bounds.left(), uv_bounds.bottom());
        Vertex2f(bounds.left(), bounds.bottom());
        TexCoord2f(uv_bounds.right(), uv_bounds.bottom());
        Vertex2f(bounds.right(), bounds.bottom());
        TexCoord2f(uv_bounds.right(), uv_bounds.top());
        Vertex2f(bounds.right(), bounds.top());
        TexCoord2f(uv_bounds.left(), uv_bounds.top());
        Vertex2f(bounds.left(), bounds.top());
        End();
    }
}