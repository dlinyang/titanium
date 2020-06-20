use super::object::*;
use crate::base::camera::Camera;
use std::collections::HashMap;
//
#[derive(Clone)]
pub struct Scene {
    pub name: String,
    pub objects: Vec<String>,
    pub meshes: Vec<String>,
    pub lights: Vec<LightObject>,
    pub cameras: Vec<CameraObject>,
    pub data: HashMap<String,Object>,
}

#[derive(Clone,Copy)]
pub enum SceneErr {
    ExistObject,
    NoExitObject,
    Atomic,
}

impl Scene {
    pub fn new(name: String) -> Self {
        Scene {
            name,
            objects: Vec::new(),
            meshes: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn insert_object(&mut self, object: Object) -> Result<(),SceneErr> {
        if self.data.contains_key(&object.name) {
            //exit a same name object
            Err(SceneErr::ExistObject)
        } else {
            match &object.sub_objects {
                SubObject::Atomic(primitive_object) => {
                    if let Some(parent) = object.parent.clone() {
                        if let Some(parent_object) = self.data.get_mut(&parent) {
                            match &mut parent_object.sub_objects {
                                SubObject::Atomic(prim) => match prim {
                                    PrimitiveObject::Empty => {
                                        parent_object.sub_objects = SubObject::Discreteness{ node_names: vec![parent.clone()]};
                                        self.data.insert(object.name.clone(), object);
                                        Ok(())
                                    },
                                    //can‘t insert to parent which is  not a empty atomic object
                                    _ => Err(SceneErr::Atomic),
                                }
                                SubObject::Discreteness {node_names} => {
                                    match primitive_object {
                                        PrimitiveObject::Empty => (),
                                        PrimitiveObject::Data(_) => self.meshes.push(object.name.clone()),
                                    }
                                    node_names.push(object.name.clone());
                                    self.data.insert(object.name.clone(), object);
                                    Ok(())
                                },
                            }
                        } else {
                            Err(SceneErr::NoExitObject)
                        }
                    } else {
                        match primitive_object {
                            PrimitiveObject::Empty => (),
                            PrimitiveObject::Data(_) => self.meshes.push(object.name.clone()),
                        }
                        self.objects.push(object.name.clone());
                        self.data.insert(object.name.clone(), object);
                        Ok(())
                    }
                },
                //can't insert  a discreteness object
                _ => Err(SceneErr::Atomic)
            }
        }
    }

    pub fn get_camera(&self, name: String) -> Option<Camera> {
        let mut result: Option<Camera> = None;

        for camera_object in &self.cameras {
            if camera_object.name == name {
                result = Some(camera_object.camera.clone());
                break;
            }
        }

        result
    }

    pub fn add_camera(&mut self,name: String, camera: Camera) {
        self.cameras.push(CameraObject { name, camera});
    }
}