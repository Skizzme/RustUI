use std::fs::{read_to_string};
use std::time::Instant;
use gl11::*;
use gl11::types::*;
use crate::gl20::{UniformMatrix4fv, VertexAttribPointer};
use crate::shader::Shader;

#[derive(Debug)]
pub struct Renderer {
    rounded_rect_shader: Shader,
    pub texture_shader: Shader,
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
        }
    }

    pub unsafe fn update(&self) {
        let mut model_view_projection_matrix: [f32; 16] = [0.0; 16];
        gl::GetFloatv(crate::gl20::PROJECTION_MATRIX, model_view_projection_matrix.as_mut_ptr());
        UniformMatrix4fv(self.texture_shader.get_uniform_location("u_projection"), 1, gl::FALSE, model_view_projection_matrix.as_ptr().cast());
    }

    pub unsafe fn draw_rounded_rect(&self, left: f32, top: f32, right: f32, bottom: f32, radius: f32, color: u32) {
        Enable(BLEND);
        Enable(TEXTURE_2D);
        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.u_put_float("u_size", vec![right-left, bottom-top]);
        self.rounded_rect_shader.u_put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.u_put_float("u_color", self.get_rgb(color));

        self.draw_texture_rect(left, top, right, bottom, 0xffffffff);

        self.rounded_rect_shader.unbind();
        Disable(TEXTURE_2D);
        Disable(BLEND);
    }

    pub unsafe fn draw_rect(&self, left: f32, top: f32, right: f32, bottom: f32, color: u32) {
        Enable(BLEND);
        self.set_color(color);
        Begin(QUADS);
        Vertex2d(left as GLdouble, bottom as GLdouble);
        Vertex2d(right as GLdouble, bottom as GLdouble);
        Vertex2d(right as GLdouble, top as GLdouble);
        Vertex2d(left as GLdouble, top as GLdouble);
        End();
        Disable(BLEND);
    }

    pub unsafe fn draw_texture_rect(&self, left: f32, top: f32, right: f32, bottom: f32, color: u32) {
        Enable(TEXTURE_2D);
        Begin(QUADS);
        self.set_color(color);
        TexCoord2d(0.0, 1.0);
        Vertex2d(left as GLdouble, bottom as GLdouble);
        TexCoord2d(1.0, 1.0);
        Vertex2d(right as GLdouble, bottom as GLdouble);
        TexCoord2d(1.0, 0.0);
        Vertex2d(right as GLdouble, top as GLdouble);
        TexCoord2d(0.0, 0.0);
        Vertex2d(left as GLdouble, top as GLdouble);
        End();
    }

    pub unsafe fn draw_texture_rect_uv(&self, left: f32, top: f32, right: f32, bottom: f32, uv_left: f64, uv_top: f64, uv_right: f64, uv_bottom: f64, color: u32) {
        Enable(TEXTURE_2D);
        Begin(QUADS);
        self.set_color(color);
        TexCoord2d(uv_left, uv_bottom);
        Vertex2d(left as GLdouble, bottom as GLdouble);
        TexCoord2d(uv_right, uv_bottom);
        Vertex2d(right as GLdouble, bottom as GLdouble);
        TexCoord2d(uv_right, uv_top);
        Vertex2d(right as GLdouble, top as GLdouble);
        TexCoord2d(uv_left, uv_top);
        Vertex2d(left as GLdouble, top as GLdouble);
        End();
    }

    pub unsafe fn set_color(&self, color: u32) {
        let alpha = (color >> 24 & 255) as f32 / 255f32;
        let red = (color >> 16 & 255) as f32 / 255f32;
        let green = (color >> 8 & 255) as f32 / 255f32;
        let blue = (color & 255) as f32 / 255f32;
        Color4d(red as GLdouble, green as GLdouble, blue as GLdouble, alpha as GLdouble);
    }

    pub fn get_rgb(&self, color: u32) -> Vec<f32> {
        let alpha = (color >> 24 & 255) as f32 / 255f32;
        let red = (color >> 16 & 255) as f32 / 255f32;
        let green = (color >> 8 & 255) as f32 / 255f32;
        let blue = (color & 255) as f32 / 255f32;
        vec![red, green, blue, alpha]
    }
}