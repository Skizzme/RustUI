use gl::*;
use gl::types::*;

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture_id: GLuint,
    pub width: i32,
    pub height: i32,
    pub format: GLenum,
}

impl Texture {
    pub unsafe fn create(width: i32, height: i32, bytes: &Vec<u8>, format: GLenum, interpolation: GLenum) -> Self {
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
        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, interpolation as i32);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, interpolation as i32);
        Texture {
            texture_id: tex_id,
            width,
            height,
            format,
        }
    }

    pub unsafe fn set_texture(&mut self, bytes: &Vec<u8>) {
        self.bind();
        TexSubImage2D(
            TEXTURE_2D,
            0,
            0, 0,
            self.width, self.height,
            self.format,
            UNSIGNED_BYTE,
            bytes.as_slice().as_ptr().cast()
        );
        Texture::unbind();
    }

    pub unsafe fn bind(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
    }

    pub unsafe fn unbind() {
        BindTexture(TEXTURE_2D, 0);
    }
}