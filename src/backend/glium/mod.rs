#[macro_use]

pub mod application;
pub mod event;
pub mod renderer;

pub use application::*;
pub use event::*;
pub use renderer::*;

use glium::implement_vertex;
use crate::base::vertex::{Vertex, Position, ImagePosition};

implement_vertex!(Vertex, position, normal, tex_coordinate);
implement_vertex!(Position, position);
implement_vertex!(ImagePosition, position, tex_coordinate);
