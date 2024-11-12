use gl::*;
use gl::types::*;

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture_id: GLuint,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub unsafe fn create(width: i32, height: i32, bytes: &Vec<u8>, format: GLenum) -> Self {
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
        }
    }

    pub unsafe fn bind(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
    }

    pub unsafe fn unbind() {
        BindTexture(TEXTURE_2D, 0);
    }
}