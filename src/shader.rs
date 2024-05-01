use gl11::*;
use crate::gl20::*;
use crate::gl20::types::*;
// use gl;
// use gl::*;
// use gl::types::{GLfloat, GLint};

#[derive(Default)]
pub struct Shader {
    vertex_shader: u32,
    fragment_shader: u32,
    program: u32,
    pub created: bool,
    vertex_source: String,
    fragment_source: String,
}

impl Shader {
    pub unsafe fn new(vertex_source: String, fragment_source: String) -> Self {
        let mut shader = Shader {
            vertex_shader: CreateShader(VERTEX_SHADER),
            fragment_shader: CreateShader(FRAGMENT_SHADER),
            program: CreateProgram(),
            created: false,
            vertex_source,
            fragment_source,
        };

        let v_res = compile(shader.vertex_shader, shader.vertex_source.as_str());
        if v_res != 1 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0i32;

            GetShaderInfoLog(shader.vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Vertex Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
        let f_res = compile(shader.fragment_shader, shader.fragment_source.as_str());
        if f_res != 1 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0i32;

            GetShaderInfoLog(shader.fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Fragment Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }

        AttachShader(shader.program, shader.vertex_shader);
        AttachShader(shader.program, shader.fragment_shader);

        LinkProgram(shader.program);
        let mut linked = 0;
        GetProgramiv(shader.program, LINK_STATUS, &mut linked);

        if linked != 1 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0i32;

            GetProgramInfoLog(shader.program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));

        }
        DeleteShader(shader.vertex_shader);
        DeleteShader(shader.fragment_shader);

        println!("Successfully created shader");
        shader.created = true;

        return shader;
    }

    pub unsafe fn bind(&self) {
        UseProgram(self.program);
    }

    pub unsafe fn unbind(&self) {
        UseProgram(0);
    }

    pub unsafe fn put_int(&self, name: &str, data: Vec<u32>) {
        // println!("{:?}, {:?}", name, data);
        let loc = GetUniformLocation(self.program, name.as_bytes().as_ptr().cast());
        match data.len() {
            1 => {
                Uniform1i(loc, *data.get(0).unwrap() as GLint);
            }
            2 => {
                Uniform2i(loc, *data.get(0).unwrap() as GLint, *data.get(1).unwrap() as GLint);
            }
            3 => {
                Uniform3i(loc, *data.get(0).unwrap() as GLint, *data.get(1).unwrap() as GLint, *data.get(2).unwrap() as GLint);
            }
            4 => {
                Uniform4i(loc, *data.get(0).unwrap() as GLint, *data.get(1).unwrap() as GLint, *data.get(2).unwrap() as GLint, *data.get(3).unwrap() as GLint);
            }
            _ => {}
        }
    }

    pub unsafe fn put_float(&self, name: &str, data: Vec<f32>) {
        // println!("{:?}, {:?}", name, data);
        let loc = GetUniformLocation(self.program, name.as_bytes().as_ptr().cast());
        match data.len() {
            1 => {
                Uniform1f(loc, *data.get(0).unwrap() as GLfloat);
            }
            2 => {
                Uniform2f(loc, *data.get(0).unwrap() as GLfloat, *data.get(1).unwrap() as GLfloat);
            }
            3 => {
                Uniform3f(loc, *data.get(0).unwrap() as GLfloat, *data.get(1).unwrap() as GLfloat, *data.get(2).unwrap() as GLfloat);
            }
            4 => {
                Uniform4f(loc, *data.get(0).unwrap() as GLfloat, *data.get(1).unwrap() as GLfloat, *data.get(2).unwrap() as GLfloat, *data.get(3).unwrap() as GLfloat);
            }
            _ => {}
        }
    }
}

unsafe fn compile(shader: u32, source: &str) -> GLint {
    ShaderSource(shader, 1, &source.as_bytes().as_ptr().cast(), &source.len().try_into().unwrap());
    CompileShader(shader);
    let mut success = 0;
    GetShaderiv(shader, COMPILE_STATUS, &mut success);
    success
}