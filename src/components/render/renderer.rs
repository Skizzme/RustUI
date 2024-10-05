use std::cell::RefCell;
use std::path;
use std::rc::Rc;

use gl::*;

use crate::asset_manager::file_contents_str;
use crate::components::render::bounds::{Bounds, ToBounds};
use crate::components::render::color::ToColor;
use crate::components::render::stack::Stack;
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl11::types::GLenum;
use crate::gl_binds::gl30::{Begin, End, PROJECTION_MATRIX, TexCoord2d, TexCoord2f, Vertex2f};

/// The global renderer to render basically everything non-text related
///
/// Uses mostly immediate GL, so best for a simple UI
#[derive(Debug)]
pub struct Renderer {
    rounded_rect_shader: Shader,
    pub texture_shader: Shader,
    pub mask_shader: Shader,
    pub circle_shader: Shader,
    stack: Stack
}

impl Renderer {

    pub unsafe fn new() -> Self {
        Renderer {
            rounded_rect_shader: Shader::new(
                file_contents_str("shaders/rounded_rect/vertex.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (rounded_rect/vertex.glsl)"),
                file_contents_str("shaders/rounded_rect/fragment.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (rounded_rect/fragment.glsl)"),
            ),
            texture_shader: Shader::new(
                file_contents_str("shaders/test_n/vertex.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (test_n/vertex.glsl)"),
                file_contents_str("shaders/test_n/fragment.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (test_n/fragment.glsl)"),
            ),
            mask_shader: Shader::new(
                file_contents_str("shaders/mask/vertex.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (mask/vertex.glsl)"),
                file_contents_str("shaders/mask/fragment.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (mask/fragment.glsl)"),
            ),
            circle_shader: Shader::new(
                file_contents_str("shaders/circle/vertex.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (mask/vertex.glsl)"),
                file_contents_str("shaders/circle/fragment.glsl".replace("/", path::MAIN_SEPARATOR_STR)).expect("Failed to read shader file (mask/fragment.glsl)"),
            ),
            stack: Stack::new(),
        }
    }

    pub unsafe fn end_frame(&mut self) {
        self.stack.clear();
    }

    pub fn stack(&mut self) -> &mut Stack {
        &mut self.stack
    }

    /// Should be called every frame, and whenever the matrix needs to be stored and sent to shaders
    pub unsafe fn get_transform_matrix(&self) -> [f64; 16] {
        let mut matrix: [f64; 16] = [0.0; 16];
        GetDoublev(PROJECTION_MATRIX, matrix.as_mut_ptr());
        matrix
    }

    /// Draws a nice rounded rectangle using texture shaders
    pub unsafe fn draw_rounded_rect(&mut self, mut bounds: impl ToBounds, radius: f32, color: impl ToColor) {
        let mut bounds = bounds.to_bounds() + Bounds::ltrb(-0.5, -0.5, 0.5, 0.5); // correct for blending created by the shader
        self.stack.begin();
        self.stack.push(Blend(true));
        self.stack.push(Texture2D(true));

        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.u_put_float("u_size", vec![bounds.width(), bounds.height()]);
        self.rounded_rect_shader.u_put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.u_put_float("u_color", color.to_color().rgba());

        self.draw_texture_rect(&bounds, 0xffffffff);

        self.rounded_rect_shader.unbind();

        self.stack.end();
    }

    /// Draws a circle, using a rounded rect, with the center point at x, y
    pub unsafe fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: impl ToColor) {
        self.draw_rounded_rect(Bounds::ltrb(x-radius, y-radius, x+radius, y+radius), radius, color);
    }

    /// The most boring rectangle
    pub unsafe fn draw_rect(&mut self, bounds: impl ToBounds, color: impl ToColor) {
        let bounds = bounds.to_bounds();
        self.stack.push(Texture2D(false));
        self.stack.push(Blend(true));

        color.apply_color();
        Begin(QUADS);
        Vertex2f(bounds.left(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.top());
        Vertex2f(bounds.left(), bounds.top());
        End();

        self.stack.pop();
        self.stack.pop();
    }

    /// A rectangle where each corner's color can be different
    ///
    /// Colors are in order of: bottom-left, bottom-right, top-right, top-left
    pub unsafe fn draw_gradient_rect(&self, bounds: &Bounds, color: (impl ToColor, impl ToColor, impl ToColor, impl ToColor)) {
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
    }

    /// Draws only the outline of a rectangle
    pub unsafe fn draw_rect_outline(&mut self, bounds: &Bounds, width: f32, color: impl ToColor) {
        self.stack.push(Texture2D(false));

        color.apply_color();
        LineWidth(width);
        Begin(LINE_STRIP);
        Vertex2f(bounds.left(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.bottom());
        Vertex2f(bounds.right(), bounds.top());
        Vertex2f(bounds.left(), bounds.top());
        Vertex2f(bounds.left(), bounds.bottom());
        End();

        self.stack.pop()
    }

    /// Draws a texture rectangle using normal UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect(&mut self, bounds: &Bounds, color: impl ToColor) {
        self.stack.push(Texture2D(false));

        Disable(TEXTURE_2D);
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

        self.stack.pop()
    }

    /// Draws a texture rectangle using specified UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect_uv(&mut self, bounds: &Bounds, uv_bounds: &Bounds, color: impl ToColor) {
        self.stack.push(Texture2D(true));

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

        self.stack.pop()
    }
}

pub trait RendererWrapped {
    // fn stack(&self) -> &mut Stack;
    unsafe fn get_transform_matrix(&self) -> [f64; 16];
    unsafe fn draw_rounded_rect(&mut self, bounds: impl ToBounds, radius: f32, color: impl ToColor);
    unsafe fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: impl ToColor);
    unsafe fn draw_rect(&mut self, bounds: impl ToBounds, color: impl ToColor);
    unsafe fn draw_gradient_rect(&self, bounds: &Bounds, color: (impl ToColor, impl ToColor, impl ToColor, impl ToColor));
    unsafe fn draw_rect_outline(&mut self, bounds: &Bounds, width: f32, color: impl ToColor);
    unsafe fn draw_texture_rect(&mut self, bounds: &Bounds, color: impl ToColor);
    unsafe fn draw_texture_rect_uv(&mut self, bounds: &Bounds, uv_bounds: &Bounds, color: impl ToColor);
}

// #[derive(Debug, Clone)]
// pub struct RendererWrapped {
//     renderer: Rc<RefCell<Renderer>>
// }

impl RendererWrapped for Rc<RefCell<Renderer>> {
    // fn stack(&self) -> &mut Stack {
    //     &mut self.borrow_mut().stack
    // }

    /// Should be called every frame, and whenever the matrix needs to be stored and sent to shaders
    unsafe fn get_transform_matrix(&self) -> [f64; 16] {
        self.borrow_mut().get_transform_matrix()
    }

    /// Draws a nice rounded rectangle using texture shaders
    unsafe fn draw_rounded_rect(&mut self, mut bounds: impl ToBounds, radius: f32, color: impl ToColor) {
        self.borrow_mut().draw_rounded_rect(bounds, radius, color);
    }

    /// Draws a circle, using a rounded rect, with the center point at x, y
    unsafe fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: impl ToColor) {
        self.borrow_mut().draw_circle(x, y, radius, color);
    }

    /// The most boring rectangle
    unsafe fn draw_rect(&mut self, bounds: impl ToBounds, color: impl ToColor) {
        self.borrow_mut().draw_rect(bounds, color);
    }

    /// A rectangle where each corner's color can be different
    ///
    /// Colors are in order of: bottom-left, bottom-right, top-right, top-left
    unsafe fn draw_gradient_rect(&self, bounds: &Bounds, color: (impl ToColor, impl ToColor, impl ToColor, impl ToColor)) {
        self.borrow_mut().draw_gradient_rect(bounds, color);
    }

    /// Draws only the outline of a rectangle
    unsafe fn draw_rect_outline(&mut self, bounds: &Bounds, width: f32, color: impl ToColor) {
        self.borrow_mut().draw_rect_outline(bounds, width, color);
    }

    /// Draws a texture rectangle using normal UV coordinates
    ///
    /// The texture should be bound before calling this
    unsafe fn draw_texture_rect(&mut self, bounds: &Bounds, color: impl ToColor) {
        self.borrow_mut().draw_texture_rect(bounds, color);
    }

    /// Draws a texture rectangle using specified UV coordinates
    ///
    /// The texture should be bound before calling this
    unsafe fn draw_texture_rect_uv(&mut self, bounds: &Bounds, uv_bounds: &Bounds, color: impl ToColor) {
        self.borrow_mut().draw_texture_rect_uv(bounds, uv_bounds, color);
    }
}