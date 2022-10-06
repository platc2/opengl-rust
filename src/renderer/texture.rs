use std::ffi;

use gl::types::{GLenum, GLint, GLsizei, GLuint};
use stb_image::image::LoadResult;

use crate::renderer::texture::ImageLoadingError::{InvalidImage, TooLarge, UnsupportedFormat};

#[derive(Debug)]
pub enum ImageLoadingError {
    /// If the image is not valid
    InvalidImage(String),
    /// If the image has an unsupported format
    UnsupportedFormat,
    /// If the image is too large
    TooLarge,
}

type Result<T> = std::result::Result<T, ImageLoadingError>;

pub struct Texture {
    handle: GLuint,
}

struct Image {
    gl_type: GLenum,
    ptr: *const ffi::c_void,
    width: GLsizei,
    height: GLsizei,
    depth: usize,
}

impl Image {
    /// # Errors
    /// - [`Error::TooLarge`]
    pub fn from_byte(image: &stb_image::image::Image<u8>) -> Result<Self> {
        Ok(Self {
            gl_type: gl::UNSIGNED_BYTE,
            ptr: image.data.as_ptr().cast::<ffi::c_void>(),
            width: Self::convert_dimension(image.width)?,
            height: Self::convert_dimension(image.height)?,
            depth: image.depth,
        })
    }

    /// # Errors
    /// - [`Error::TooLarge`]
    pub fn from_float(image: &stb_image::image::Image<f32>) -> Result<Self> {
        Ok(Self {
            gl_type: gl::FLOAT,
            ptr: image.data.as_ptr().cast::<ffi::c_void>(),
            width: Self::convert_dimension(image.width)?,
            height: Self::convert_dimension(image.height)?,
            depth: image.depth,
        })
    }

    fn convert_dimension(dimension: usize) -> Result<GLsizei> {
        GLsizei::try_from(dimension)
            .map_err(|_| TooLarge)
    }
}

impl Texture {
    /// # Errors
    /// - [`Error::InvalidImage`]
    /// - [`Error::UnsupportedFormat`]
    pub fn from(image_data: &mut [u8]) -> Result<Self> {
        let mut handle: GLuint = 0;

        // TODO - Figure out why glTextureParameteri requires Glint while these values are GLenum
        let gl_linear = unsafe { GLint::try_from(gl::LINEAR).unwrap_unchecked() };
        let gl_rgba = unsafe { GLint::try_from(gl::RGBA).unwrap_unchecked() };

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1 as GLsizei, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TextureParameteri(handle, gl::TEXTURE_MIN_FILTER, gl_linear);
            gl::TextureParameteri(handle, gl::TEXTURE_MAG_FILTER, gl_linear);
        }

        // TODO - Figure out how to inline stb_image into match expression without the value being dropped too early
        let stb_image = stb_image::image::load_from_memory(image_data);
        let image_data = match &stb_image {
            LoadResult::Error(error) => Err(InvalidImage(error.to_string())),
            LoadResult::ImageU8(image_data) => Ok(Image::from_byte(image_data)?),
            LoadResult::ImageF32(image_data) => Ok(Image::from_float(image_data)?),
        }?;

        let format = format_from_depth(image_data.depth)?;

        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0 as GLint, gl_rgba, image_data.width,
                           image_data.height, 0 as GLint, format, image_data.gl_type, image_data.ptr);
            gl::GenerateTextureMipmap(handle);
        }
        Ok(Self { handle })
    }

    pub const fn handle(&self) -> GLuint {
        self.handle
    }
}

const fn format_from_depth(depth: usize) -> Result<GLenum> {
    match depth {
        1 => Ok(gl::RED),
        2 => Ok(gl::RG),
        3 => Ok(gl::RGB),
        4 => Ok(gl::RGBA),
        _ => Err(UnsupportedFormat),
    }
}
