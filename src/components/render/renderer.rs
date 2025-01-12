use std::path;

use gl::*;

use crate::components::context::context;
use crate::components::render::color::ToColor;
use crate::components::render::stack::Stack;
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl30::{Begin, End, PROJECTION_MATRIX, TexCoord2d, TexCoord2f, Vertex2f};
use crate::test_ui::asset_manager::file_contents_str;

/// The global renderer to render basically everything non-text related
///
/// Uses mostly immediate GL, so best for a simple UI
#[derive(Debug)]
pub struct Renderer {
    rounded_rect_shader: Shader,
    pub texture_shader: Shader,
    pub mask_shader: Shader,
    pub circle_shader: Shader,
    pub blend_shader: Shader,
    /// up, down
    pub blur_shaders: (Shader, Shader),
    /// up, down
    pub bloom_shaders: (Shader, Shader),
    pub blur_fb: u32,
    stack: Stack
}

pub fn shader_file(path: impl ToString) -> String {
    let path = path.to_string();
    file_contents_str(path.replace("/", path::MAIN_SEPARATOR_STR)).expect(format!("Failed to read shader file ({})", path).as_str()).to_string()
}

impl Renderer {

    pub unsafe fn new() -> Self {
        let default_vert = shader_file("shaders/vertex.glsl");
        Renderer {
            rounded_rect_shader: Shader::new(shader_file("shaders/rounded_rect/vertex.glsl"), shader_file("shaders/rounded_rect/fragment.glsl")),
            texture_shader: Shader::new(shader_file("shaders/test_n/vertex.glsl"), shader_file("shaders/test_n/fragment.glsl")),
            mask_shader: Shader::new(shader_file("shaders/mask/vertex.glsl"), shader_file("shaders/mask/fragment.glsl")),
            circle_shader: Shader::new(shader_file("shaders/circle/vertex.glsl"), shader_file("shaders/circle/fragment.glsl")),
            blur_shaders: (
                Shader::new(default_vert.clone(), shader_file("shaders/blur/blur_up.frag")),
                Shader::new(default_vert.clone(), shader_file("shaders/blur/blur_down.frag")),
            ),
            bloom_shaders: (
                Shader::new(default_vert.clone(), shader_file("shaders/bloom/bloom_up.frag")),
                Shader::new(default_vert.clone(), shader_file("shaders/bloom/bloom_down.frag")),
            ),
            blend_shader: Shader::new(default_vert.clone(), shader_file("shaders/fb_blend.frag")),
            blur_fb: 0,
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
    pub unsafe fn draw_rounded_rect(&mut self, vec4: impl Into<Vec4>, radius: f32, color: impl ToColor) {
        let vec4 = vec4.into() + Vec4::ltrb(-0.5, -0.5, 0.5, 0.5); // correct for blending created by the shader
        self.stack.begin();
        self.stack.push(Blend(true));
        self.stack.push(Texture2D(true));

        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.u_put_float("u_size", vec![vec4.width(), vec4.height()]);
        self.rounded_rect_shader.u_put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.u_put_float("u_color", color.to_color().rgba().to_vec());

        self.draw_texture_rect(&vec4, 0xffffffff);

        Shader::unbind();

        self.stack.end();
    }

    /// Draws a circle, using a rounded rect, with the center point at x, y
    pub unsafe fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: impl ToColor) {
        self.draw_rounded_rect(Vec4::ltrb(x-radius, y-radius, x+radius, y+radius), radius, color);
    }

    /// The most boring rectangle
    pub unsafe fn draw_rect(&mut self, vec4: impl Into<Vec4>, color: impl ToColor) {
        let vec4 = vec4.into();
        self.stack.push(Texture2D(false));
        self.stack.push(Blend(true));

        color.apply_color();
        Begin(QUADS);
        Vertex2f(vec4.left(), vec4.bottom());
        Vertex2f(vec4.right(), vec4.bottom());
        Vertex2f(vec4.right(), vec4.top());
        Vertex2f(vec4.left(), vec4.top());
        End();

        self.stack.pop();
        self.stack.pop();
    }

    /// A rectangle where each corner's color can be different
    ///
    /// Colors are in order of: bottom-left, bottom-right, top-right, top-left
    pub unsafe fn draw_gradient_rect(&self, vec4: impl Into<Vec4>, color: (impl ToColor, impl ToColor, impl ToColor, impl ToColor)) {
        let vec4 = vec4.into();
        Begin(QUADS);
        color.0.apply_color();
        Vertex2f(vec4.left(), vec4.bottom());
        color.1.apply_color();
        Vertex2f(vec4.right(), vec4.bottom());
        color.2.apply_color();
        Vertex2f(vec4.right(), vec4.top());
        color.3.apply_color();
        Vertex2f(vec4.left(), vec4.top());
        End();
    }

    /// Draws only the outline of a rectangle
    pub unsafe fn draw_rect_outline(&mut self, vec4: impl Into<Vec4>, width: f32, color: impl ToColor) {
        let vec4 = vec4.into();
        self.stack.push(Texture2D(false));

        color.apply_color();
        LineWidth(width);
        Begin(LINE_STRIP);
        Vertex2f(vec4.left(), vec4.bottom());
        Vertex2f(vec4.right(), vec4.bottom());
        Vertex2f(vec4.right(), vec4.top());
        Vertex2f(vec4.left(), vec4.top());
        Vertex2f(vec4.left(), vec4.bottom());
        End();

        self.stack.pop();
    }

    /// Draws a texture rectangle using normal UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect(&mut self, vec4: impl Into<Vec4>, color: impl ToColor) {
        let vec4 = vec4.into();
        self.stack.push(Texture2D(false));

        Disable(TEXTURE_2D);
        Begin(QUADS);
        color.apply_color();
        TexCoord2d(0.0, 1.0);
        Vertex2f(vec4.left(), vec4.bottom());
        TexCoord2d(1.0, 1.0);
        Vertex2f(vec4.right(), vec4.bottom());
        TexCoord2d(1.0, 0.0);
        Vertex2f(vec4.right(), vec4.top());
        TexCoord2d(0.0, 0.0);
        Vertex2f(vec4.left(), vec4.top());
        End();

        self.stack.pop();
    }

    /// Draws a texture rectangle using specified UV coordinates
    ///
    /// The texture should be bound before calling this
    pub unsafe fn draw_texture_rect_uv(&mut self, vec4: impl Into<Vec4>, uv_vec4: impl Into<Vec4>, color: impl ToColor) {
        let vec4 = vec4.into();
        let uv_vec4 = uv_vec4.into();
        self.stack.push(Texture2D(true));

        Begin(QUADS);
        color.apply_color();
        TexCoord2f(uv_vec4.left(), uv_vec4.bottom());
        Vertex2f(vec4.left(), vec4.bottom());
        TexCoord2f(uv_vec4.right(), uv_vec4.bottom());
        Vertex2f(vec4.right(), vec4.bottom());
        TexCoord2f(uv_vec4.right(), uv_vec4.top());
        Vertex2f(vec4.right(), vec4.top());
        TexCoord2f(uv_vec4.left(), uv_vec4.top());
        Vertex2f(vec4.left(), vec4.top());
        End();

        self.stack.pop();
    }

    pub unsafe fn draw_screen_rect(&mut self) {
        self.draw_texture_rect_uv(&context().window().bounds(), &Vec4::ltrb(0.0, 0.0, 1.0, 1.0), 0xffffffff);
    }
    pub unsafe fn draw_screen_rect_flipped(&mut self) {
        self.draw_texture_rect_uv(&context().window().bounds(), &Vec4::ltrb(0.0, 1.0, 1.0, 0.0), 0xffffffff);
    }
}