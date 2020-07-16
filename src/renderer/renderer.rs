use crate::base::camera::Camera;
use super::scene::RenderScene;
use super::light::Light;
use super::pipeline::*;
use super::image::ImageData;
use rmu::raw::{Vec4f,Vec2f};

/// Renderer for 3d scene
pub trait Renderer {
    fn backgroud_color(&mut self, color: Vec4f);
    fn update_camera(&mut self, camera: &Camera);
    fn update_scene(&mut self, scene: &mut RenderScene);
    //
    fn update_light(&mut self, name: &String, light: &Light);
    fn remove_light(&mut self, name: &String);
    //
    fn update_texture(&mut self, data: ImageData, name: &String);
    fn remove_texture(&mut self, name: &String);
    //
    fn clear(&mut self);
    fn render(&mut self);
    fn shadow_map(&mut self);
    fn swap_buffer(&mut self);
    //
    fn set_antialiasing(&mut self,enable: bool);
    fn set_hdr(&mut self, enable: bool);
    fn set_gamma(&mut self, gamma: f32);
}

use super::Text;

/// Renderer for 2d canvas
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
pub trait Renderer2D {
    fn load_font(&mut self, name: &str, path: &str);
    //
    fn draw_points(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32);
    fn draw_line(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32);
    fn draw_lines(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32);
    fn draw_polygon(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32);
    fn draw_polygon_fill(&mut self, positions: Vec<Vec2f>, color: Vec4f);
    fn set_pixel(&mut self, position: Vec2f, color: Vec4f);
    /// positons [top_left,top right, bottom_right, bottom_left]
    fn draw_image(&mut self, data: ImageData, positions: [Vec2f;4], uv: [Vec2f;4]);
    fn draw_text(&mut self, text: &Text, color: Vec4f);
    /// load_sprite() ≡  update_texture()
    fn load_sprite(&mut self, data: ImageData, name: &String);
    /// remove_sprite() ≡ remove_texture()
    fn remove_sprite(&mut self, name: &String);
    fn draw_sprite(&mut self, name: &String, position: [Vec2f;4], uv: [Vec2f;4]);
}

pub trait RenderPassRenderer<S,P> {
    fn render_pass(&mut self, material_name: &String, uniform_data: &mut S, render_pass: &RenderPass<P>);
}