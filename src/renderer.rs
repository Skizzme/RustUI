use std::fs::{File, read, read_to_string};
use gl11::*;
use gl11::types::*;
use crate::shader::Shader;

pub struct Renderer {
    rounded_rect_shader: Shader
}

impl Renderer {

    pub unsafe fn new() -> Self {
        Renderer {
            rounded_rect_shader: Shader::new(read_to_string("src\\resources\\shaders\\rounded_rect\\vertex_shader.glsl").unwrap(),
                                             read_to_string("src\\resources\\shaders\\rounded_rect\\fragment_shader.glsl").unwrap())
        }
    }

    pub unsafe fn draw_rounded_rect(&self, left: f32, top: f32, right: f32, bottom: f32, radius: f32, color: u32) {
        Enable(BLEND);
        Enable(TEXTURE_2D);
        self.rounded_rect_shader.bind();
        self.rounded_rect_shader.put_float("u_size", vec![right-left, bottom-top]);
        self.rounded_rect_shader.put_float("u_radius", vec![radius]);
        self.rounded_rect_shader.put_float("u_color", self.get_rgb(color));

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
        TexCoord2d(0.0, 0.0);
        Vertex2d(left as GLdouble, top as GLdouble);
        TexCoord2d(1.0, 0.0);
        Vertex2d(right as GLdouble, top as GLdouble);
        TexCoord2d(1.0, 1.0);
        Vertex2d(right as GLdouble, bottom as GLdouble);
        TexCoord2d(0.0, 1.0);
        Vertex2d(left as GLdouble, bottom as GLdouble);
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