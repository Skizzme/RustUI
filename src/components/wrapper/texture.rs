use std::mem::size_of_val;
use std::ptr;

use gl::*;
use gl::types::*;

use crate::components::context::context;
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl11::{EnableClientState, TexCoordPointer, TEXTURE_COORD_ARRAY, VertexPointer};
use crate::gl_binds::gl30::Color4d;

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture_id: GLuint,
    pub width: i32,
    pub height: i32,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub uvo: GLuint,
    pub ebo: GLuint,
}

impl Texture {
    pub unsafe fn create(width: i32, height: i32, bytes: &Vec<u8>, format: GLenum) -> Self {
        // let shader = Shader::new(asset_manager::file_contents_str ("shaders/test_n/vertex.glsl").unwrap(), asset_manager::file_contents_str ("shaders/test_n/fragment.glsl").unwrap());

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
        let vertices = [[10.0, height as f32 / 10.0], [width as f32 / 10.0, height as f32 / 10.0], [width as f32 / 10.0, 10.0], [10.0, 10.0],];

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(vertices.as_slice()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            STATIC_DRAW
        );
        EnableClientState(crate::gl_binds::gl11::VERTEX_ARRAY);
        VertexPointer(2, crate::gl_binds::gl11::FLOAT, 4 * 2, ptr::null());

        EnableClientState(TEXTURE_COORD_ARRAY);
        let uvs: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
        BindBuffer(ARRAY_BUFFER, uvo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(uvs.as_slice()) as GLsizeiptr,
            uvs.as_ptr() as *const _,
            STATIC_DRAW
        );
        TexCoordPointer(2, crate::gl_binds::gl11::FLOAT, 4 * 2, ptr::null());

        let indices = [0, 1, 2, 0, 2, 3];
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            STATIC_DRAW
        );

        let mut tex_id = 0;
        GenTextures(1, &mut tex_id);
        BindTexture(TEXTURE_2D, tex_id);

        TexImage2D(
            TEXTURE_2D,
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
            width,
            height,
            vao,
            vbo,
            uvo,
            ebo,
        }
    }

    pub unsafe fn render(&self) {
        Enable(BLEND);
        Enable(TEXTURE_2D);

        context().renderer().texture_shader.bind();
        ActiveTexture(crate::gl_binds::gl20::TEXTURE0);

        self.bind();
        BindVertexArray(self.vao);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);
        Texture::unbind();

        Shader::unbind();
        BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
        BindBuffer(ARRAY_BUFFER, 0);
    }

    pub unsafe fn draw(&self) {
        Enable(TEXTURE_2D);
        self.bind();
        BindVertexArray(self.vao);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);
        Texture::unbind();
    }

    pub unsafe fn bind(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
    }

    pub unsafe fn unbind() {
        BindTexture(TEXTURE_2D, 0);
    }
}