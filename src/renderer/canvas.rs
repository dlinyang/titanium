/// Canvas is a 2d graphics Renderer 
/// ----------------------------
/// 2d coodinate system
/// 
/// ```ignore
/// (0.0) ------ (0,1)
///  |             |
///  |             |
///  |             |
/// (1,0) ------ (1,1)
/// ```
pub trait Canvas: GraphicsPaint + TextPaint{
    fn init(&mut self); // ≡ clear();
    fn finish(&mut self); // ≡ swapbuffer();
}

use rmu::raw::{Vec2f,Vec4f};
use super::image::Image;

pub trait GraphicsPaint {
    fn set_size(&mut self, size: f32);
    fn set_line_width(&mut self, size:f32);
    fn set_color(&mut self, color: Vec4f);

    fn draw_points(&mut self, positions: Vec<Vec2f>);
    fn draw_line(&mut self, positions: Vec<Vec2f>);
    fn draw_polygon(&mut self, position: Vec<Vec2f>);
    fn draw_polygon_fill(&mut self, position: Vec<Vec2f>);
    fn draw_image(&mut self, position_uvs: Vec<(Vec2f,Vec2f)>, sprite_name: &str);

    fn load_sprite(&mut self, name: &str, image: &Image);
    fn remove_sprite(&mut self, name: &str);
}

use crate::base::utils::Size;

pub type Scale = Size;

pub enum Align {
    Left,
    Center,
    Right,
}

pub trait TextPaint {
    fn load_font(&mut self, name: &str, path: &str);
    fn set_font(&mut self, name: &str);
    fn set_font_scale(&mut self, scale: Scale);
    fn set_font_color(&mut self, color: Vec4f);
    fn set_font_space(&mut self, font_space: f32);
    fn set_line_space(&mut self, ling_space: f32);
    fn set_align(&mut self, align: Align);
    fn set_max_width(&mut self, max_width: f32);
    fn set_max_line(&mut self, max_line: u32);
    fn set_position(&mut self, position: Vec2f);
    fn draw_text(&mut self, text: &str);
}