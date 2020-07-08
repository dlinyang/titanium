#[derive(Clone,Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

use rmu::raw::{Vec4f,Vec3f};

impl RGBA {
    pub fn rgb(&self) -> Vec3f {
        [self.r as f32, self.g as f32, self.b as f32]
    }
}
/// # color table
pub const BLACK: Vec4f = [0.0,0.0,0.0,1.0];
pub const WHITE: Vec4f = [1.0,1.0,1.0,1.0];
pub const GREY: Vec4f = [ 190.0/255.0, 190.0/255.0, 190.0/255.0, 1.0];
pub const GREY11: Vec4f = [28.0/255.0, 28.0/255.0, 28.0/255.0, 1.0];
pub const GREY21: Vec4f = [ 54.0/255.0, 54.0/255.0, 54.0/255.0, 1.0];
pub const GREY31: Vec4f = [ 79.0/255.0, 79.0/255.0, 79.0/255.0, 1.0];