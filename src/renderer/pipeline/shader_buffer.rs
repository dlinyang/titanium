///shader
pub use crate::base::material::Material;

pub trait ShaderBuffer<T> {
    fn shader(&self, material_name: &String) -> Option<&T>;
}