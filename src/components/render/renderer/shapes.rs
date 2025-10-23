use std::ptr;
use gl::{DrawArraysInstanced, ARRAY_BUFFER, DEBUG_OUTPUT, FALSE, FLOAT, QUADS};
use num_traits::NumCast;
use crate::components::context::context;
use crate::components::render::color::{Color, To4Colors, ToColor};
use crate::components::render::renderer::Renderable;
use crate::components::render::stack::State::{Blend, Texture2D};
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl11::{Disable, DrawElements, Enable, CULL_FACE, DEPTH_TEST, TRIANGLES, UNSIGNED_INT};
use crate::gl_binds::gl11::types::{GLsizei, GLuint};
use crate::gl_binds::gl20::ELEMENT_ARRAY_BUFFER;
use crate::gl_binds::gl30::{Begin, BindVertexArray, End, Vertex2f, FRAMEBUFFER_SRGB};
use crate::gl_binds::gl41::DrawElementsInstanced;

pub struct Rect {
    vec4: Vec4,
    color: (Color, Color, Color, Color),
    radius: f32,
    vao: VertexArray,
}

impl Rect {
    unsafe fn create_vao(vec4: &Vec4) -> VertexArray {
        let shader = &mut context().renderer().rounded_rect_shader_2;

        let mut vao = VertexArray::new();
        vao.bind();

        let vert_data = vec![
            vec4.left(), vec4.top(), 0., 0.,
            vec4.right(), vec4.top(), 1., 0.,
            vec4.right(), vec4.bottom(), 1., 1.,
            vec4.left(), vec4.bottom(), 0., 1.,
        ];

        let mut vertices = Buffer::new(ARRAY_BUFFER);
        vertices.set_values(&vert_data);
        vertices.attribPointerStride(shader.get_attrib_location("vert") as GLuint, 4, FLOAT, FALSE, 0);

        let mut indices = Buffer::new(ELEMENT_ARRAY_BUFFER);
        indices.set_values(&vec![0, 1, 2, 0, 2, 3]);
        indices.bind();

        vao.add_buffer(vertices);
        vao.add_buffer(indices);

        VertexArray::unbind();

        vao
    }

    pub unsafe fn new(vec4: impl Into<Vec4>, colors: impl To4Colors) -> Self {
        let vec4 = vec4.into();

        Self {
            vec4: vec4.clone(),
            color: colors.to_colors(),
            radius: 0.,
            vao: Self::create_vao(&vec4)
        }
    }

    pub unsafe fn set_bounds(&mut self, vec4: &Vec4) {
        self.vao.delete();
        self.vao = Self::create_vao(vec4);
        self.vec4 = vec4.clone();
    }

    pub fn set_radius(&mut self, v: impl NumCast) {
        self.radius = v.to_f32().unwrap_or(0.);
    }

    pub fn set_colors(&mut self, colors: impl To4Colors) {
        self.color = colors.to_colors();
    }

    pub(super) unsafe fn draw_rect(&self) {
        let renderer = context().renderer();
        let vec4 = self.vec4 + Vec4::ltrb(-0.5, -0.5, 0.5, 0.5); // correct for blending created by the shader
        renderer.stack.begin();
        renderer.stack.push(Blend(true));
        renderer.stack.push(Texture2D(true));

        let shader = &renderer.rounded_rect_shader_2;
        shader.bind();
        shader.u_put_float("u_size", vec![vec4.width(), vec4.height()]);
        shader.u_put_float("u_radius", vec![self.radius]);
        shader.u_put_float("u_color_lt", self.color.0.rgba().to_vec());
        shader.u_put_float("u_color_rt", self.color.1.rgba().to_vec());
        shader.u_put_float("u_color_lb", self.color.2.rgba().to_vec());
        shader.u_put_float("u_color_rb", self.color.3.rgba().to_vec());

        self.vao.bind();

        DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
        // DrawArraysInstanced(gl::TRIANGLES, 0, 6, 1);

        VertexArray::unbind();

        Shader::unbind();

        renderer.stack.end();
        context().framework().mark_layer_dirty(&vec4);
        // vec4.debug_draw(0xffff10ff);
    }

    pub fn vec4(&self) -> Vec4 {
        self.vec4
    }

    pub fn color(&self) -> (Color, Color, Color, Color) {
        self.color
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Renderable for Rect {
    unsafe fn pre_render(&mut self) {

    }

    unsafe fn render(&self) {
        self.draw_rect()
    }
}