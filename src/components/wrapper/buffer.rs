use std::ptr;
use gl::{ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, DeleteBuffers, DYNAMIC_DRAW, ELEMENT_ARRAY_BUFFER, FLOAT, GenBuffers, GenVertexArrays};
use gl::types::GLsizeiptr;
use crate::gl_binds::gl11::FALSE;
use crate::gl_binds::gl11::types::{GLboolean, GLenum, GLint, GLsizei, GLuint};
use crate::gl_binds::gl20::{EnableVertexAttribArray, VertexAttribPointer};
use crate::gl_binds::gl30::DeleteVertexArrays;
use crate::gl_binds::gl41::VertexAttribDivisor;

pub struct Buffer {
    gl_ref: u32,
    gl_type: GLenum,
    type_size: usize,
}

impl Buffer {
    pub unsafe fn new(gl_type: GLenum) -> Self {
        let mut gl_ref = 0;
        GenBuffers(1, &mut gl_ref);
        Buffer {
            gl_ref,
            gl_type,
            type_size: 0,
        }
    }
    pub unsafe fn set_values<T: Sized>(&mut self, v: Vec<T>) {
        self.bind();
        BufferData(
            self.gl_type,
            (v.len() * size_of::<T>()) as GLsizeiptr,
            v.as_ptr() as *const _,
            DYNAMIC_DRAW,
        );
        self.type_size = size_of::<T>();
        self.unbind();
    }

    pub unsafe fn attribPointer(&self, attrib: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, divisor: GLuint) {
        self.bind();
        EnableVertexAttribArray(attrib);
        VertexAttribPointer(attrib, size, type_, normalized, self.type_size as GLint, ptr::null());
        VertexAttribDivisor(attrib, divisor);
        self.unbind();
    }

    pub unsafe fn delete(self) {
        DeleteBuffers(1, &self.gl_ref);
    }

    pub unsafe fn bind(&self) {
        BindBuffer(self.gl_type, self.gl_ref);
    }

    pub unsafe fn unbind(&self) {
        BindBuffer(self.gl_type, 0);
    }

    pub fn gl_ref(&self) -> u32 {
        self.gl_ref
    }
    pub fn gl_type(&self) -> GLenum {
        self.gl_type
    }
}

pub struct VertexArray {
    gl_ref: u32,
    buffers: Vec<Buffer>
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut gl_ref = 0;
        GenVertexArrays(1, &mut gl_ref);
        VertexArray {
            gl_ref,
            buffers: vec![],
        }
    }

    pub unsafe fn delete(self) {
        for b in self.buffers {
            b.delete();
        }
        DeleteVertexArrays(1, &self.gl_ref);
    }

    pub unsafe fn bind(&self) {
        BindVertexArray(self.gl_ref);
    }

    pub unsafe fn unbind() {
        BindVertexArray(0);
    }

    pub unsafe fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
    }
    pub fn gl_ref(&self) -> u32 {
        self.gl_ref
    }
    pub fn buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }
}