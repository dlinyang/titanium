use crate::base::camera::Camera;
use super::canvas::Canvas;
use super::scene::RenderScene;
use super::light::Light;
use super::pipeline::*;
use rmu::raw::{Vec4f,Vec2f};

//the definition of Renderer
pub trait Renderer {
    fn backgroud_color(&mut self, color: Vec4f);
    fn update_camera(&mut self, camera: &Camera);
    fn update_canvas(&mut self, canvas: &mut Canvas);
    fn update_scene(&mut self, scene: &mut RenderScene);
    fn update_light(&mut self, name: &String, light: &Light);
    fn clear(&mut self);
    fn render(&mut self);
    fn shadow_map(&mut self);
    fn render_scene(&mut self);
    fn render_canvas(&mut self);
    fn swap_buffer(&mut self);
    fn set_antialiasing(&mut self,enable: bool);
    fn set_hdr(&mut self, enable: bool);
    fn set_gamma(&mut self, gamma: f32);
}

use crate::base::Position;

pub trait Renderer2D {
    fn draw_points(&mut self, vertex: Vec<Position>, color: Vec4f, size: f32);
    fn draw_line(&mut self, vertex: Vec<Position>, color: Vec4f, size: f32);
    fn draw_lines(&mut self, vertex: Vec<Position>, color: Vec4f, size: f32);
    fn draw_ploygon(&mut self, vertex: Vec<Position>, color: Vec4f, size: f32);
    fn draw_ploygon_fill(&mut self, vertex: Vec<Position>, color: Vec4f);
    fn set_pixel(&mut self, position: Vec2f, color: Vec4f);
    fn draw_image<T>(&mut self, data: [T], w: u32, h: f32);
}

pub trait RenderPassRenderer<S,C,P> {
    fn scene_render_pass(&mut self, material_name: &String, uniform_data: &mut S, render_pass: &RenderPass<P>);
    fn canvas_render_pass(&mut self, layer_index: LayerIndex, uniform_data: &mut C, render_pass: &RenderPass<P>);
}

pub enum LayerIndex {
    Text(usize),
    Graphics(usize),
}