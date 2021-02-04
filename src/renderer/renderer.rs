use crate::base::camera::Camera;
use super::light::Light;
use super::pipeline::*;
use super::image::Image;
use super::data::{Object,GMesh};
use rmu::raw::Vec4f;
use crate::base::material::Material;

/// Renderer for 3d scene
pub trait Renderer: RenderProdure + RendererManager {
    fn background_color(&mut self, color: Vec4f);
    fn set_antialiasing(&mut self,enable: bool);
    fn set_hdr(&mut self, enable: bool);
    fn set_gamma(&mut self, gamma: f32);
}

pub trait RenderProdure {
    fn clear(&mut self);
    fn shadow_map(&mut self);
    fn render(&mut self);
    fn swap_buffer(&mut self);
}

pub trait RendererManager {
    fn update_camera(&mut self, camera: &Camera);
    fn update_mesh(&mut self, name: &str, mesh: &GMesh);
    fn update_material(&mut self, name: &str, material: &Material);
    fn update_object(&mut self, name: &str, object: &mut Object);
    //
    fn update_light(&mut self, name: &str, light: &Light);
    fn remove_light(&mut self, name: &str);
    //
    fn update_texture(&mut self, name: &str, image: &Image);
    fn remove_texture(&mut self, name: &str);
}

pub trait RenderPassRenderer<S,P> {
    fn render_pass(&mut self, material_name: &String, uniform_data: &mut S, render_pass: &RenderPass<P>);
}