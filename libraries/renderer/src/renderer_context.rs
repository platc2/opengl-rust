use std::ffi::c_void;

use sdl2::Sdl;
use sdl2::video::{GLContext, GLProfile, Window, WindowBuildError};
use thiserror::Error;

use gl_bindings as gl;

use crate::renderer_context::Error::{ContextInit, SdlInit, VideoSubsystemInit, WindowTooLarge};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to initialize SDL: {0}")]
    SdlInit(String),

    #[error("Failed to initialize video subsystem: {0}")]
    VideoSubsystemInit(String),

    #[error("Failed to initialize window: {0}")]
    WindowInit(#[from] WindowBuildError),

    #[error("Failed to initialize context: {0}")]
    ContextInit(String),

    #[error("Window dimension are too large")]
    WindowTooLarge,
}

type Result<T> = std::result::Result<T, Error>;

pub struct RendererContext {
    sdl: Sdl,
    window: Window,
    _gl_context: GLContext,
}

pub struct WindowDimension {
    pub width: usize,
    pub height: usize,
}

impl WindowDimension {
    #[must_use]
    pub const fn of(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Default for WindowDimension {
    fn default() -> Self {
        Self {
            width: 900,
            height: 700,
        }
    }
}

pub struct OpenGLVersion {
    major: u8,
    minor: u8,
}

impl Default for OpenGLVersion {
    fn default() -> Self {
        Self::of(4, 1)
    }
}

impl OpenGLVersion {
    #[must_use]
    pub const fn of(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl RendererContext {
    /// # Errors
    /// - SDL failed to initialise
    /// - SDL video subsystem failed to initialise
    /// - Failed to create SDL window
    /// - Failed to initialise OpenGL context
    pub fn init(
        window_title: &str,
        window_dimension: &WindowDimension,
        opengl_version: &OpenGLVersion,
    ) -> Result<Self> {
        let sdl = sdl2::init().map_err(SdlInit)?;
        let video_subsystem = sdl.video().map_err(VideoSubsystemInit)?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_major_version(opengl_version.major);
        gl_attr.set_context_minor_version(opengl_version.minor);
        gl_attr.set_context_flags().debug().set();
        let window = video_subsystem
            .window(
                window_title,
                u32::try_from(window_dimension.width).map_err(|_| WindowTooLarge)?,
                u32::try_from(window_dimension.height).map_err(|_| WindowTooLarge)?,
            )
            .opengl()
            .resizable()
            .build()?;
        let gl_context = window.gl_create_context().map_err(ContextInit)?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<c_void>());

        Ok(Self {
            sdl,
            window,
            _gl_context: gl_context,
        })
    }

    #[must_use]
    pub const fn sdl(&self) -> &Sdl {
        &self.sdl
    }

    #[must_use]
    pub const fn window(&self) -> &Window {
        &self.window
    }
}
