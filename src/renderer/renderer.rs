use crate::base::camera::Camera;
use super::canvas::Canvas;
use super::scene::RenderScene;
use super::light::Light;
use rmu::raw::Vec4f;

//the definition of Renderer
pub trait Renderer {
    fn backgroud_color(&mut self, color: Vec4f);
    fn update_camera(&mut self, camera: &Camera);
    fn update_canvas(&mut self, canvas: &mut Canvas);
    fn update_scene(&mut self, scene: &mut RenderScene);
    fn update_light(&mut self, name: &String, light: &Light);
    fn clear(&mut self);
    fn render_scene(&mut self);
    fn render_canvas(&mut self);
    //fn render(&mut self, render_Data);
    fn swap_buffer(&mut self);
}