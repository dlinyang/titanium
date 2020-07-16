/// Graphics Generator
pub mod rectangle;
pub mod round_rectangle;
pub mod circle;

pub use rectangle::*;
pub use round_rectangle::*;

use rmu::raw::Vec2f;

pub trait Graphics2d {
    fn positions(&self) -> Vec<Vec2f>;
}