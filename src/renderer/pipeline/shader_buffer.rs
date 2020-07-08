///shader
pub use crate::base::material::Material;
pub use super::render_pass::RenderPass;

pub trait ShaderBuffer<T> {
    fn shader(&self, material_name: &String) -> Option<&RenderPass<T>>;
}