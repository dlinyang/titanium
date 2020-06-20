use crate::base::transform::Transform;
use crate::base::mesh::Mesh;

#[derive(Clone)]
pub enum PrimitiveObject {
    Empty,
    Data(Mesh),
}

#[derive(Clone)]
pub enum SubObject {
    Atomic(PrimitiveObject),
    Discreteness{ node_names: Vec<String> },
}

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub parent: Option<String>,
    pub transform: Transform,
    pub sub_objects: SubObject,
}

impl Object {
    
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            transform: Transform::new(),
            sub_objects: SubObject::Atomic(PrimitiveObject::Empty),
        }

    }

    pub fn from(name: String, primitive_object: PrimitiveObject) -> Self {
        Self {
            name,
            parent: None,
            transform: Transform::new(),
            sub_objects: SubObject::Atomic(primitive_object),
        }
    }

}

use crate::base::light::Light;
#[derive(Clone)]
pub struct LightObject {
    pub name: String,
    pub light: Light,
}

use crate::base::camera::Camera;
#[derive(Clone)]
pub struct CameraObject {
    pub name: String,
    pub camera: Camera,
}