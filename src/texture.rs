use std::fs::read_to_string;
use std::mem::{size_of, size_of_val};
use std::ptr;
use std::ptr::null;
use std::rc::Rc;
use gl::{ALPHA, ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, DrawElements, ELEMENT_ARRAY_BUFFER, EnableVertexAttribArray, FALSE, FLOAT, GenBuffers, GenTextures, GenVertexArrays, GetFloatv, LINEAR, REPEAT, RGB, STATIC_DRAW, TexImage2D, TexParameteri, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, TRIANGLES, UNSIGNED_BYTE, VertexAttribPointer};
use gl11::{PushMatrix, TEXTURE_2D};
use gl::types::{GLint, GLuint};
use crate::gl20::{BindTexture, CLAMP_TO_EDGE, Enable, MODELVIEW_MATRIX, PopMatrix, PROJECTION_MATRIX, RGBA, Rotated, Translated, UniformMatrix4fv, UNSIGNED_INT, VertexAttrib1f};
use crate::gl20::types::{GLenum, GLsizeiptr};
use crate::renderer::Renderer;
use crate::shader::Shader;

#[derive(Debug)]
pub struct Texture {
    pub texture_id: GLuint,
    pub renderer: Rc<Renderer>,
    pub width: i32,
    pub height: i32,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub uvo: GLuint,
    pub ebo: GLuint,
}

impl Texture {
    pub unsafe fn create(renderer: Rc<Renderer>, width: i32, height: i32, bytes: Vec<u8>, format: GLenum) -> Self {
        let shader = Shader::new(read_to_string("src\\resources\\shaders\\test\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\test\\fragment.glsl").unwrap());

        let mut vao = 0;
        let mut vbo = 0;
        let mut uvo = 0;
        let mut ebo = 0;
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut uvo);
        GenBuffers(1, &mut ebo);
        BindVertexArray(vao);

        // Make buffers
        // let vertices = [[-0.5f32, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5],];
        let vertices = [[0.0, height as f32], [width as f32, height as f32], [width as f32, 0.0], [0.0, 0.0],];

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(vertices.as_slice()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            STATIC_DRAW
        );

        let uvs: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
        BindBuffer(ARRAY_BUFFER, uvo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(uvs.as_slice()) as GLsizeiptr,
            uvs.as_ptr() as *const _,
            STATIC_DRAW
        );

        let indices = [0, 1, 2, 2, 3, 0];
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            STATIC_DRAW
        );

        // Bind buffers
        BindBuffer(ARRAY_BUFFER, vbo);
        let position1 = renderer.texture_shader.get_attrib_location("position") as GLuint;
        VertexAttribPointer(
            position1,
            2,
            FLOAT,
            FALSE,
            size_of::<[f32; 2]>().try_into().unwrap(),
            0 as *const _,
        );
        EnableVertexAttribArray(position1);

        BindBuffer(ARRAY_BUFFER, uvo);
        let position2 = renderer.texture_shader.get_attrib_location("vertexTexCoord") as GLuint;
        VertexAttribPointer(
            position2,
            2,
            FLOAT,
            FALSE,
            size_of::<[f32; 2]>().try_into().unwrap(),
            0 as *const _,
        );
        EnableVertexAttribArray(position2);

        let mut tex_id = 0;
        GenTextures(1, &mut tex_id);
        BindTexture(TEXTURE_2D, tex_id);

        TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as GLint,
            width,
            height,
            0,
            format,
            UNSIGNED_BYTE,
            bytes.as_slice().as_ptr().cast(),
        );

        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
        Texture {
            texture_id: tex_id,
            renderer,
            width,
            height,
            vao,
            vbo,
            uvo,
            ebo,
        }
    }

    pub unsafe fn render(&self) {
        Enable(TEXTURE_2D);
        self.bind();
        self.renderer.texture_shader.bind();
        // let mut model_view_projection_matrix: [f32; 16] = [0.0; 16];
        // GetFloatv(PROJECTION_MATRIX, model_view_projection_matrix.as_mut_ptr());
        // UniformMatrix4fv(self.shader.get_uniform_location("u_projection"), 1, FALSE, model_view_projection_matrix.as_ptr().cast());

        self.renderer.update();
        // VertexAttribPointer()
        // self.renderer.draw_texture_rect(x, y, x+width, y+height, 0xffffffff);
        BindVertexArray(self.vao);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);
        self.renderer.texture_shader.unbind();
        self.unbind();
    }

    pub unsafe fn draw(&self) {
        Enable(TEXTURE_2D);
        self.bind();
        BindVertexArray(self.vao);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);
        self.unbind();
    }

    pub unsafe fn bind(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
        // BindVertexArray(self.vao);
        // DrawElements(TRIANGLES, 6, UNSIGNED_BYTE, null());
        // BindVertexArray(0);
    }

    pub unsafe fn unbind(&self) {
        BindTexture(TEXTURE_2D, 0);
        // BindVertexArray(self.vao);
        // DrawElements(TRIANGLES, 6, UNSIGNED_BYTE, null());
        // BindVertexArray(0);
    }
}