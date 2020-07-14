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
    pub shadow_maps: Vec<Sampler<'a,DepthTexture2d>>,
    pub shadow_map_views: Vec<Mat4f>,
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
        shadow_maps: Vec<Sampler<'a,DepthTexture2d>>, 
        shadow_map_views: Vec<Mat4f>,
        view_position: Vec3f, 
        hdr_enable   : bool, 
        gamma        : f32
    ) -> Self {

        Self {
            matrix,
            lights,
            lights_count,
            shadow_maps,
            shadow_map_views,
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

            let behavior = SamplerBehavior {
                depth_texture_comparison: Some(DepthTextureComparison::LessOrEqual),
                magnify_filter: MagnifySamplerFilter::Nearest,
                minify_filter: MinifySamplerFilter::Nearest,
                ..Default::default()
            };

            let mut i = 0;

            /* issue: depth texture as uniform have some problem will make shadow mapping don't work*/
            while i < self.data.shadow_maps.len() {
                output(format!("shadow[{}].tex,",i).as_str(),self.data.shadow_maps[i].as_uniform_value());
                output(format!("shadow[{}].mvp,",i).as_str(),self.data.shadow_map_views[i].as_uniform_value());
                i += 1;
            }

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

/* Canvas rendering is draw call by Layer, material need load when rendering different layer*/
pub struct CanvasUniformData {
    pub material_property: Vec<(String, PropertyValue)>,
    pub render_target: Option<String>,
    pub multiple_render_target: Vec<String>,
}

impl CanvasUniformData {
    pub fn new(material_property: Vec<(String, PropertyValue)>) -> Self {
        Self {
            material_property,
            render_target: None,
            multiple_render_target: Vec::new(),
        }
    }
}

pub struct CanvasUniform<'a>{
    pub font_texture: Option<&'a Texture2d>,
    pub material_property_mapped: Vec<(String,PropertyValueMapped<'a>)>,
    pub render_target: Option<(String,&'a Texture2d)>,
    pub multiple_render_target: Vec<(String,&'a Texture2d)>,
}

impl<'a> CanvasUniform<'a> {
    pub fn new(
        data          : &'a CanvasUniformData, 
        font_texture  : Option<&'a Texture2d>, 
        texture_buffer: &'a HashMap<String,Texture2d>
    ) -> Self {
        let mut material_property_mapped = Vec::new();

        for (name, value) in data.material_property.iter() {
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
            font_texture,
            material_property_mapped,
            render_target,
            multiple_render_target,
            
        }
    }
}

impl <'n> Uniforms for CanvasUniform<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output : F) {
        if let Some(tex) = &self.font_texture {
            output("font_tex", tex.as_uniform_value());
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
            output(name, UniformValue::Texture2d(&tex,None));
        }

        for (name,tex) in self.multiple_render_target.iter() {
            output(name, UniformValue::Texture2d(&tex,None));
        }
    }
}

/* post pressing is use for deferred rendering or screen filter */
pub struct PostPressingUniform<'a> {
    pub data                  : Option<&'a SceneUniformData<'a>>,
    pub screen_resolution     : Vec2f,
    pub lighting              : bool,
    pub render_target         : Option<(String,&'a Texture2d)>,
    pub multiple_render_target: Vec<(String,&'a Texture2d)>,
}

impl<'a> PostPressingUniform<'a> {
    pub fn scene(
        data: &'a SceneUniformData<'a>, 
        screen_resolution: Vec2f,
        lighting: bool,
        texture_buffer: &'a HashMap<String,Texture2d>,
    ) -> Self {

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
            data: Some(data),
            screen_resolution,
            lighting,
            render_target,
            multiple_render_target,
        }
    }

    pub fn canvas(
        data: &'a CanvasUniformData, 
        screen_resolution: Vec2f,
        lighting: bool,
        texture_buffer: &'a HashMap<String,Texture2d>,
    ) -> Self {

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
            data: None,
            screen_resolution,
            lighting,
            render_target,
            multiple_render_target,
        }
    }
}

impl <'n> Uniforms for PostPressingUniform<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output : F) {
        if let Some(data) = &self.data {
            if self.lighting {
                output("view_position", data.view_position.as_uniform_value());
                output("hdr_enable"   , data.hdr_enable.as_uniform_value());
                output("gamma"        , data.gamma.as_uniform_value());
                output("Lights"       , data.lights.as_uniform_value());
                output("lights_count" , data.lights_count.as_uniform_value());
            }
        }

        output("screen_resolution", self.screen_resolution.as_uniform_value());

        if let Some((name,tex)) = &self.render_target {
            output(name, UniformValue::Texture2d(&tex,None));
        }

        for (name,tex) in self.multiple_render_target.iter() {
            output(name, UniformValue::Texture2d(&tex,None));
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