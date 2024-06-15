use std::time::Instant;

pub use glfw;
pub use glfw::{Context, WindowMode};
// use gl;
// use gl::{ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, GenBuffers, GenVertexArrays, MULTISAMPLE, STATIC_DRAW};
pub use image::EncodableLayout;
use winapi::um::wincon::FreeConsole;

use crate::default_screen::DefaultScreen;
pub use crate::screen::GuiScreen;
pub use crate::window::Window;

pub mod renderer;
pub mod shader;
pub mod screen;
pub mod animation;
pub mod font;
pub mod default_screen;
pub mod window;
pub mod texture;
pub mod gl30;