use super::renderer::GLRenderer;
use super::buffer::*;
use std::rc::Rc;
use glium::{
    buffer::*,
    texture::*,
    index::{IndexBuffer,PrimitiveType},
    vertex::VertexBuffer,
};
use crate::base::{
    camera::Camera,
    material::Material,
    index::Indices,
};
use crate::renderer::{
    RendererManager,
    data::*,
    image::*,
    light::*,
};

impl RendererManager for GLRenderer {
        fn update_object(&mut self, name: &str, object: &mut Object) {

        if let Some(scene_buffer) = Rc::get_mut(&mut self.data_buffer.scene_buffer) {

            let name = name.to_string();
            match object.update {
                DataUpdate::ALL => {
                    scene_buffer.objects.insert(name.clone(),RenderObject::new(object.mesh_name.clone(), object.material_name.clone(), object.transform));

                    use std::collections::HashMap;
                    if let Some(same_material_objects) = scene_buffer.same_material_objects.get_mut(&object.material_name) {
                        same_material_objects.insert(name.clone(),());
                    } else {
                        scene_buffer.same_material_objects.insert(object.material_name.clone(),HashMap::new());
                        scene_buffer.same_material_objects.get_mut(&object.material_name).unwrap().insert(name.clone(),());
                    }
                },
                DataUpdate::Transfrom => {
                    if let Some(render_object) = scene_buffer.objects.get_mut(&name) {
                        render_object.transform = object.transform;
                    }
                },
                DataUpdate::Material => {
                    if let Some(render_object) = scene_buffer.objects.get_mut(&name) {
                        render_object.material_name = object.material_name.clone();
                    }
                },
                DataUpdate::Statue => {

                },
                DataUpdate::Not => (),
            }
            // update  finished
            object.update = DataUpdate::Not;
        } else {

        }

    }

    fn update_texture(&mut self, name: &str, image: &Image) {
        
        use std::borrow::Cow;

        let texture = Texture2d::with_format(
            &self.display,
            RawImage2d {
                data: Cow::Owned(image.data.clone()),
                width: image.dimensions.0,
                height: image.dimensions.1,
                format: image.image_type.into(),
            },
            image.image_type.into(),
            MipmapsOption::NoMipmap,
        ).unwrap();

        self.data_buffer.texture_buffer.insert(name.to_string(), texture);

    }

    fn update_mesh(&mut self, name: &str, mesh: &GMesh) {
        if let Some(scene_buffer) = Rc::get_mut(&mut self.data_buffer.scene_buffer) {

            let vertex_buffer = VertexBuffer::new(&self.display, &mesh.vertices).unwrap();

            let index_buffer = match &mesh.indices {
                Indices::Points(indices) => 
                    IndexBuffer::new(&self.display, PrimitiveType::Points, &indices).unwrap(),
                Indices::EdgeLists(indices) => 
                    IndexBuffer::new(&self.display, PrimitiveType::LinesList, &indices).unwrap(),
                Indices::TriangleFace(indices) => 
                    IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &indices).unwrap(),
            };

            scene_buffer.meshes.insert(name.to_string(), RenderMesh::new(vertex_buffer, index_buffer));
        } 
    }

    fn update_material(&mut self,name: &str, material: &Material) {
        if let Some(scene_buffer) = Rc::get_mut(&mut self.data_buffer.scene_buffer) {
            scene_buffer.materials.insert(name.to_string(), material.clone());
        }
    }

    fn remove_texture(&mut self, name: &str) {
        self.data_buffer.texture_buffer.remove(name);
    }

    fn update_camera(&mut self, camera: &Camera) {
        self.data_buffer.camera = camera.clone();
    }

    fn update_light(&mut self, name: &str, light: &Light) {
        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        if let None = light_buffer.lights.insert(name.to_string(), light.clone()) {
            use glium::texture::*;

            let shadow_map_size = light_buffer.shadow_map_size;
            let shadow_map = 
                DepthTexture2d::empty(&self.display, shadow_map_size, shadow_map_size).unwrap();
            light_buffer.shadow_maps.insert(name.to_string(), shadow_map);

            let lights: Vec<Light> =light_buffer.lights.values().map(|x| {*x}).collect();
            light_buffer.buffer = 
                Buffer::new(
                    &self.display, 
                    lights.as_slice(), 
                    BufferType::UniformBuffer, 
                    BufferMode::default()
                ).unwrap();
        }
    }

    fn remove_light(&mut self, name: &str) {
        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        if let Some(_) = light_buffer.lights.remove(name) {
            light_buffer.shadow_maps.remove(name);
            let lights: Vec<Light> =light_buffer.lights.values().map(|x| {*x}).collect();
            light_buffer.buffer = 
                Buffer::new(
                    &self.display, 
                    lights.as_slice(), 
                    BufferType::UniformBuffer, 
                    BufferMode::default()
                ).unwrap();
        }
    }
}

use glium::texture::{ClientFormat,UncompressedFloatFormat};

impl From<ImageType> for ClientFormat {
    fn from(ty: ImageType) -> Self {
        match ty {
            ImageType::U8 => ClientFormat::U8,
            ImageType::U8U8U8 => ClientFormat::U8U8U8,
            ImageType::U8U8U8U8 => ClientFormat::U8U8U8U8,
        }
    }
}

impl From<ImageType> for UncompressedFloatFormat {
    fn from(ty: ImageType) -> Self {
        match ty {
            ImageType::U8 => UncompressedFloatFormat::U8,
            ImageType::U8U8U8 => UncompressedFloatFormat::U8U8U8,
            ImageType::U8U8U8U8 => UncompressedFloatFormat::U8U8U8U8,
        }
    }
}