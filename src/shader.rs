use gl::*;
use gl::types::*;

/// A wrapper for a GL program containing a vertex and fragment shaders
#[derive(Default, Debug)]
pub struct Shader {
    vertex_shader: u32,
    fragment_shader: u32,
    program: u32,
    pub created: bool,
    vertex_source: String,
    fragment_source: String,
}

impl Shader {
    /// Creates a shader program from the vertex and fragment sources.
    pub unsafe fn new(vertex_source: impl ToString, fragment_source: impl ToString) -> Self {
        let mut shader = Shader {
            vertex_shader: CreateShader(VERTEX_SHADER),
            fragment_shader: CreateShader(FRAGMENT_SHADER),
            program: CreateProgram(),
            created: false,
            vertex_source: vertex_source.to_string(),
            fragment_source: fragment_source.to_string(),
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

        shader.created = true;

        return shader;
    }

    /// Binds this program id
    pub unsafe fn bind(&self) {
        UseProgram(self.program);
    }

    /// Unbinds this program
    ///
    /// In other words, binds program ID 0
    pub unsafe fn unbind(&self) {
        UseProgram(0);
    }

    /// Gets the location of an attribute variable of the program
    pub unsafe fn get_attrib_location(&self, name: &str) -> GLint {
        let cname = std::ffi::CString::new(name).expect("Failed to convert to cstring");
        GetAttribLocation(self.program, cname.as_ptr())
    }

    /// Gets the location of a uniform variable of the program
    pub unsafe fn get_uniform_location(&self, name: &str) -> GLint {
        let cname = std::ffi::CString::new(name).expect("Failed to convert to cstring");
        GetUniformLocation(self.program, cname.as_ptr())
    }

    /// Sets the integer uniform of this program specified by the name
    ///
    /// `data` is the values of the uniform. 1 value is a int, 2 is a vec2 etc
    pub unsafe fn u_put_int(&self, name: &str, data: Vec<u32>) {
        let cname = std::ffi::CString::new(name).expect("Failed to convert to cstring");
        let loc = GetUniformLocation(self.program, cname.as_ptr());
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

    /// Sets the float uniform of this program specified by the name
    ///
    /// `data` is the values of the uniform. 1 value is a float, 2 is a vec2 etc
    pub unsafe fn u_put_float(&self, name: &str, data: Vec<f32>) {
        let cname = std::ffi::CString::new(name).expect("Failed to convert to cstring");
        let loc = GetUniformLocation(self.program, cname.as_ptr());
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

//     pub unsafe fn u_put_double(&self, name: &str, data: Vec<f64>) {
//         let cname = std::ffi::CString::new(name).expect("Failed to convert to cstring");
//         let loc = GetUniformLocation(self.program, cname.as_ptr());
//         match data.len() {
//             1 => {
//                 Uniform1d(loc, *data.get(0).unwrap() as GLdouble);
//             }
//             2 => {
//                 Uniform2d(loc, *data.get(0).unwrap() as GLdouble, *data.get(1).unwrap() as GLdouble);
//             }
//             3 => {
//                 Uniform3d(loc, *data.get(0).unwrap() as GLdouble, *data.get(1).unwrap() as GLdouble, *data.get(2).unwrap() as GLdouble);
//             }
//             4 => {
//                 Uniform4d(loc, *data.get(0).unwrap() as GLdouble, *data.get(1).unwrap() as GLdouble, *data.get(2).unwrap() as GLdouble, *data.get(3).unwrap() as GLdouble);
//             }
//             _ => {}
//         }
//     }
}

unsafe fn compile(shader: u32, source: &str) -> GLint {
    ShaderSource(shader, 1, &source.as_bytes().as_ptr().cast(), &source.len().try_into().unwrap());
    CompileShader(shader);
    let mut success = 0;
    GetShaderiv(shader, COMPILE_STATUS, &mut success);
    success
}