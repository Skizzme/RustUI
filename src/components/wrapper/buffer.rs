use gl::{ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, DYNAMIC_DRAW, ELEMENT_ARRAY_BUFFER, GenBuffers, GenVertexArrays};
use gl::types::GLsizeiptr;
use crate::gl_binds::gl11::types::{GLenum, GLuint};

pub struct BufferBuilder<T: Sized> {
    values: Vec<T>,
    gl_type: GLenum,
}

impl<T> BufferBuilder<T> {
    pub fn new(gl_type: GLenum) -> Self {
        BufferBuilder {
            values: vec![],
            gl_type,
        }
    }

    pub fn add_value(&mut self, v: T) {
        self.values.push(v);
    }
    pub fn set_values(&mut self, v: Vec<T>) {
        self.values = v;
    }

    pub unsafe fn build(self) -> GLuint {
        let mut id = 0;
        GenBuffers(1, &mut id);
        BindBuffer(ELEMENT_ARRAY_BUFFER, id);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (self.values.len() * size_of::<T>()) as GLsizeiptr,
            self.values.as_ptr() as *const _,
            DYNAMIC_DRAW,
        );
        id
    }
}