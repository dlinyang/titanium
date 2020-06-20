#[macro_use]

pub mod application;
pub mod event;
pub mod renderer;

pub use application::*;
pub use event::*;
pub use renderer::*;

use glium::implement_vertex;
use crate::base::vertex::{Vertex, Position};

implement_vertex!(Vertex, position, normal, tex_coordinate);
implement_vertex!(Position, position, tex_coordinate);
