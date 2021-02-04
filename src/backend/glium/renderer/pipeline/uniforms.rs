use rmu::raw::Mat4f;
use glium::uniforms::*;
use rmu::raw::{Vec2f,Vec3f,Vec4f};
use glium::buffer::*;
use glium::texture::*;
use glium::implement_uniform_block;
use crate::renderer::light::Light;
use crate::base::material::*;
use std::collections::HashMap;

#[derive(Copy,Clone)]
pub struct CameraMatrix {
    pub project: Mat4f,
    pub view: Mat4f,
}

implement_uniform_block!(CameraMatrix,project,view);


pub struct SceneUniformData<'a> {
    pub matrix: &'a UniformBuffer<CameraMatrix>,
    pub lights: &'a Buffer<[Light]>,
    pub lights_count: i32,
    pub view_position: Vec3f,
    pub hdr_enable: bool,
    pub gamma: f32,
    pub render_target: Option<String>,
    pub multiple_render_target: Vec<String>,
}

impl<'a> SceneUniformData<'a> {
    pub fn new(
        matrix       : &'a UniformBuffer<CameraMatrix>, 
        lights       : &'a Buffer<[Light]>, 
        lights_count : i32,
        view_position: Vec3f, 
        hdr_enable   : bool, 
        gamma        : f32
    ) -> Self {

        Self {
            matrix,
            lights,
            lights_count,
            view_position,
            hdr_enable,
            gamma,
            render_target: None,
            multiple_render_target: Vec::new(),
        }
    }
}

/* Because of scene rendering is draw call by same material , material property 
   need load  when differnet object rendering with same material*/
pub struct SceneUniform<'a>{
    pub data: &'a SceneUniformData<'a>,
    pub transform: Mat4f,
    pub material_property_mapped: Vec<(String,PropertyValueMapped<'a>)>,
    pub lighting: bool,
    pub render_target: Option<(String,&'a Texture2d)>,
    pub multiple_render_target: Vec<(String, &'a Texture2d)>
}

impl<'a> SceneUniform<'a> {
    pub fn new(
        data             : &'a SceneUniformData<'a>,
        transform        : Mat4f, 
        material_property: Vec<(String,PropertyValue)>, 
        lighting         : bool,
        texture_buffer   : &'a HashMap<String,Texture2d>,
    ) -> Self {
        let mut material_property_mapped = Vec::new();

        for (name, value) in material_property.iter() {
            match value {
                PropertyValue::Bool(value)  => material_property_mapped.push((name.clone(), PropertyValueMapped::Bool(*value))),
                PropertyValue::Float(value) => material_property_mapped.push((name.clone(), PropertyValueMapped::Float(*value))),
                PropertyValue::Vec2(value)  => material_property_mapped.push((name.clone(), PropertyValueMapped::Vec2(*value))),
                PropertyValue::Vec3(value)  => material_property_mapped.push((name.clone(), PropertyValueMapped::Vec3(*value))),
                PropertyValue::Vec4(value)  => material_property_mapped.push((name.clone(), PropertyValueMapped::Vec4(*value))),
                PropertyValue::Texture(value) => {
                    if let Some(tex) = texture_buffer.get(value) {
                        material_property_mapped.push((name.clone(), PropertyValueMapped::Texture(tex)));
                    }
                },
            }
        }

        let render_target = if let Some(name) = &data.render_target {
            if let Some(tex) = texture_buffer.get(name) {
                Some((name.clone(),tex))
            } else {
                None
            }
        } else {
            None
        };

        let mut multiple_render_target = Vec::new();

        for name in data.multiple_render_target.iter() {
            if let Some(tex) = texture_buffer.get(name) {
                multiple_render_target.push((name.clone(),tex));
            }
        }

        Self {
            data,
            transform,
            material_property_mapped,
            lighting,
            render_target,
            multiple_render_target,
        }
    }
}

impl<'n> Uniforms for SceneUniform<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output : F) {
        output("Camera",self.data.matrix.as_uniform_value());
        output("transform",self.transform.as_uniform_value());
        
        if self.lighting {
            output("view_position",self.data.view_position.as_uniform_value());
            output("hdr_enable", self.data.hdr_enable.as_uniform_value());
            output("gamma", self.data.gamma.as_uniform_value());
            output("Lights",self.data.lights.as_uniform_value());
            output("lights_count",self.data.lights_count.as_uniform_value());

            /* issue: depth texture as uniform have some problem will make shadow mapping don't work*/
        }

        for (name, value) in self.material_property_mapped.iter() {
            match value {
                PropertyValueMapped::Bool(value)  => output(name, value.as_uniform_value()),
                PropertyValueMapped::Float(value) => output(name, value.as_uniform_value()),
                PropertyValueMapped::Vec2(value)  => output(name, value.as_uniform_value()),
                PropertyValueMapped::Vec3(value)  => output(name, value.as_uniform_value()),
                PropertyValueMapped::Vec4(value)  => output(name, value.as_uniform_value()),
                PropertyValueMapped::Texture(value) => output(name, value.as_uniform_value()),
            }
        }

        if let Some((name,tex)) = &self.render_target {
            output(name, tex.as_uniform_value());
        }

        for (name,tex) in self.multiple_render_target.iter() {
            output(name, tex.as_uniform_value());
        }
    }
}

pub enum PropertyValueMapped<'a> {
    Bool(bool),
    Float(f32),
    Vec2(Vec2f),
    Vec3(Vec3f),
    Vec4(Vec4f),
    Texture(&'a Texture2d),
}