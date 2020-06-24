use crate::application::*;
use crate::renderer::*;
use crate::base::camera::Camera;
use crate::base::material::*;
use crate::base::font::FontSet;

use glium::{Surface, Display, DrawParameters, IndexBuffer, VertexBuffer, index, implement_uniform_block, uniforms, uniform, framebuffer::*, texture::*};
use std::collections::HashMap;

use super::buffer::*;
use super::font::*;
use super::pipeline::*;
use super::frame::Frame;

pub struct GLRenderer {
    pub display      : Display,
    pub scene_render_buffer: Texture2d,
    pub depth_buffer : DepthTexture2d,
    pub canvas_render_buffer: Texture2d,
    pub font_set     : FontSet,
    pub data_buffer  : DataBuffer,
    pub shader_buffer: GLShaderBuffer,
    pub frame        : Frame,
    pub aa_enable  : bool,
    pub config       : Config,
    pub hdr_enable   : bool,
    pub gamma        : f32,
}

impl GLRenderer {
    pub fn new(config: Config, display: Display) -> Self {
        let w = config.size.height as u32;
        let h = config.size.width as u32;

        let scene_render_buffer = Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).unwrap();

        let canvas_render_buffer = Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).unwrap();

        let depth_buffer = DepthTexture2d::empty_with_format(
            &display, 
            DepthFormat::F32, 
            MipmapsOption::NoMipmap,
            w, 
            h
        ).unwrap();

        Self {
            scene_render_buffer,
            depth_buffer,
            canvas_render_buffer,
            font_set: FontSet::new(),
            data_buffer: Default::default(),
            shader_buffer: GLShaderBuffer::new(&display),
            frame: Frame::new(&display),
            aa_enable: false,
            display,
            config,
            hdr_enable: false,
            gamma: 2.2,
        }
    }
}

use crate::renderer::canvas::graphics::GraphicsType;
use glium::index::PrimitiveType;
impl From<GraphicsType> for PrimitiveType {
    fn from(graphics_type: GraphicsType) -> PrimitiveType {
        match graphics_type {
            GraphicsType::Points      => PrimitiveType::Points,
            GraphicsType::Line        => PrimitiveType::LineStrip,
            GraphicsType::LineList    => PrimitiveType::LinesList,
            GraphicsType::Polygon     => PrimitiveType::LineLoop,
            GraphicsType::PolygonFill => PrimitiveType::TriangleFan,
        }
    }
}

use rmu::raw::Vec4f;
use crate::base::Index;
use scene::DataUpdate;
use glium::index::NoIndices;

impl Renderer for GLRenderer {

    fn backgroud_color(&mut self, color: Vec4f) {
        self.data_buffer.bg_color = color;
    }

    fn update_scene(&mut self, render_scene: &mut RenderScene) {
        // update faces
        for  (name,data) in &mut render_scene.render_data {

            match data.update {
                DataUpdate::ALL => {
                    let vertex_buffer = VertexBuffer::new(&self.display, &data.vertices).unwrap();
                    let indices = match &data.indices {
                        Index::TriangleFace(indices) => IndexBuffer::new(&self.display, index::PrimitiveType::TrianglesList, &indices).unwrap(),
                        Index::EdgeLists(indices)    => IndexBuffer::new(&self.display, index::PrimitiveType::LinesList, &indices).unwrap(),
                        Index::Points(indices)       => IndexBuffer::new(&self.display, index::PrimitiveType::Points, &indices).unwrap(),
                    };

                    let new_data = SceneData::new(vertex_buffer, indices, data.material.clone(), data.transform);
                    let old_data = self.data_buffer.scene_data.data.insert(name.clone(), new_data);

                    // if material already exits
                    if let Some(same_material_data) = self.data_buffer.scene_data.same_material_data.get_mut(&data.material.name) {
                        same_material_data.insert(name.clone(),());
                    } else {
                        // material not exist
                        self.data_buffer.scene_data.same_material_data.insert(data.material.name.clone(), HashMap::new());
                        self.data_buffer.scene_data.same_material_data.get_mut(&data.material.name).unwrap().insert(name.clone(),());
                    }

                    if let Some(old_data) = old_data {
                        self.data_buffer.scene_data.same_material_data.get_mut(&old_data.material.name).unwrap().remove(&name.clone());
                    }

                    data.update = DataUpdate::Not;
                },
                DataUpdate::Transfrom => {
                    if let Some(inner_data) = self.data_buffer.scene_data.data.get_mut(name) {
                        inner_data.transform = data.transform;
                    }

                    data.update = DataUpdate::Not;
                },
                DataUpdate::Material => {
                    if let Some(inner_data) = self.data_buffer.scene_data.data.get_mut(name) {
                        inner_data.material = data.material.clone();
                    }

                    data.update = DataUpdate::Not;
                },
                DataUpdate::Statue => {

                },
                DataUpdate::Not => (),
            }
        }
    }

    fn update_canvas(&mut self, canvas: &mut Canvas)  {

        if canvas.id != self.data_buffer.canvas_data.id {
            self.data_buffer.canvas_data.clear();
        }

        for layer in &canvas.layers {
            let mut render_layer = RenderLayer::new(layer.id);

            if let Some(text) = &layer.text {
                use rusttype::Font;
                if let Some(font_byte) = self.font_set.font_byte(&text.font) {
                    let font = Font::try_from_bytes(font_byte).unwrap();
                    render_layer.set_text(load_text(text, &font, &self.display));
                } 
            }
                        
            if let Some(graphic) = &layer.graphics {
                use crate::base::Position;
                let vertices: Vec<Position> = graphic.positions.iter().map(|v| { Position::new([2.0 * v.position[0] - 1.0, -2.0 * v.position[1] + 1.0], v.tex_coordinate)}).collect();
                let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();

                render_layer.set_graphics(GraphicsData {
                    vertex_buffer,
                    indices: NoIndices(graphic.graphics_type.into()),
                    material: graphic.material.clone(),
                })
            }

            self.data_buffer.canvas_data.update(render_layer);
            
        }

    }

    fn update_camera(&mut self, camera: &Camera) {
        self.data_buffer.view = camera.view();
        self.data_buffer.view_position = camera.look_from.into();
        self.data_buffer.project = camera.perspective();
    }

    fn update_light(&mut self, name: &String, light: &Light) {
        self.data_buffer.lights.insert(name.clone(), light.clone());
    }

    fn clear(&mut self) {
        self.clear_scene();
        self.clear_canvas();
    }

    fn clear_scene(&mut self) {
        let [r,g,b,a] = self.data_buffer.bg_color;
        let mut frame = SimpleFrameBuffer::with_depth_buffer(&self.display, &self.scene_render_buffer, &self.depth_buffer).unwrap();
        frame.clear_color_and_depth((r, g, b, a), 1.0);
    }

    fn clear_canvas(&mut self) {
        let mut frame = SimpleFrameBuffer::new(&self.display, &self.canvas_render_buffer).unwrap();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
    }

    fn render_scene(&mut self) {
        let parameters: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut frame = SimpleFrameBuffer::with_depth_buffer(&self.display, &self.scene_render_buffer, &self.depth_buffer).unwrap();

        let matrix = uniforms::UniformBuffer::new(
            &self.display, 
            CameraMatrix {
                project: self.data_buffer.project,
                view: self.data_buffer.view,
            }).unwrap();
        
        let lights = self.data_buffer.lights();

        for (material_name,same_material_data) in &self.data_buffer.scene_data.same_material_data {
            if let Some(program) = self.shader_buffer.shader(material_name) {
                for data_name in same_material_data.keys() {
                    let data = self.data_buffer.scene_data.data.get(data_name).unwrap();
                    let uniforms = SceneUniform::new(&matrix, data.transform, &lights, self.data_buffer.view_position, data.material.property());
                    frame.draw(&data.vertex_buffer, &data.indices, program, &uniforms, &parameters).unwrap();
                }
            }
        }
    }

    fn render_canvas(&mut self) {

        let parameters: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let mut frame = SimpleFrameBuffer::new(&self.display, &self.canvas_render_buffer).unwrap();

        for render_layer in &self.data_buffer.canvas_data.data {
            if let Some(graphics) = &render_layer.gragphics { 
                if let Some(program) = self.shader_buffer.shader(&graphics.material.name) {
                    frame.draw(
                        &graphics.vertex_buffer, 
                        &graphics.indices, 
                        program, 
                        &CanvasUniform::new(None,graphics.material.property()), 
                        &parameters).unwrap();
                }
            }

            if let Some(text) = &render_layer.text {
                if let Some(program) = self.shader_buffer.shader(&text.material.material_name()) {
                    frame.draw(
                    &text.vertex_buffer, 
                    &text.indices, 
                    program, 
                    &CanvasUniform::new(Some(&text.texture), text.material.property()), 
                    &parameters).unwrap();
                }
            }
        }
    }

    fn swap_buffer(&mut self)  {
        let mut target = self.display.draw();

        target.draw(
            &self.frame.vertex_buffer, 
            &self.frame.index_buffer, 
            &self.frame.program, 
            &uniform!{
                resolution: [self.config.size.width, self.config.size.height],
                scene: &self.scene_render_buffer,
                canvas: &self.canvas_render_buffer,
                aa_enable: self.aa_enable,
                gamma: self.gamma,
                hdr_enable: self.hdr_enable, 
            }, 
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
    }

    fn set_anti_aliasing(&mut self, enable: bool) {
        self.aa_enable = enable;
    }

    fn set_hdr(&mut self, enable: bool) {
        self.hdr_enable = enable;
    }

    fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }
}

use rmu::raw::Mat4f;

#[derive(Copy,Clone)]
pub struct CameraMatrix {
    pub project: Mat4f,
    pub view: Mat4f,
}

implement_uniform_block!(CameraMatrix,project,view);

use glium::uniforms::*;
use rmu::raw::Vec3f;

pub struct SceneUniform<'a>{
    pub matrix: &'a UniformBuffer<CameraMatrix>,
    pub transform: Mat4f,
    pub lights: &'a Vec<Light>,
    pub lights_count: i32,
    pub view_position: Vec3f,
    pub material_property: Vec<(&'a str,PropertyValue)>,
}

impl<'a> SceneUniform<'a> {
    pub fn new(matrix: &'a UniformBuffer<CameraMatrix>, transform: Mat4f, lights: &'a Vec<Light>, view_position: Vec3f, material_property: Vec<(&'a str,PropertyValue)>) -> Self {
        Self {
            matrix,
            transform,
            lights,
            lights_count: lights.len() as i32,
            view_position,
            material_property,
        }
    }
}

impl<'n> Uniforms for SceneUniform<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output : F) {
        output("Camera",self.matrix.as_uniform_value());
        output("transform",self.transform.as_uniform_value());
        output("view_position",self.view_position.as_uniform_value());
        let mut i: usize = 0;
        while i < self.lights.len() {
            output(format!("light[{}].is_position",i).as_str()  , self.lights[i].is_position.as_uniform_value());
            output(format!("light[{}].is_range",i).as_str()     , self.lights[i].is_range.as_uniform_value());
            output(format!("light[{}].color",i).as_str()        , self.lights[i].color.as_uniform_value());
            output(format!("light[{}].position",i).as_str()     , self.lights[i].position.as_uniform_value());
            output(format!("light[{}].direction",i).as_str()    , self.lights[i].direction.as_uniform_value());
            output(format!("light[{}].cut_off",i).as_str()      , self.lights[i].cut_off.as_uniform_value());
            output(format!("light[{}].outer_cut_off",i).as_str(), self.lights[i].outer_cut_off.as_uniform_value());
            output(format!("light[{}].linear",i).as_str()       , self.lights[i].linear.as_uniform_value());
            output(format!("light[{}].quadratic",i).as_str()    , self.lights[i].quadratic.as_uniform_value());
            i = i + 1;
        }
        output("lights_count",self.lights_count.as_uniform_value());
        for (name, value) in self.material_property.iter() {
            match value {
                PropertyValue::Bool(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Float(value) => output(name, value.as_uniform_value()),
                PropertyValue::Vec2(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Vec3(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Vec4(value)  => output(name, value.as_uniform_value()),
            }
        }
    }
}

pub struct CanvasUniform<'a>{
    pub font_texture: Option<&'a Texture2d>,
    pub material_property: Vec<(&'a str,PropertyValue)>,
}

impl<'a> CanvasUniform<'a> {
    pub fn new(font_texture: Option<&'a Texture2d>, material_property: Vec<(&'a str,PropertyValue)>) -> Self {
        Self {
            font_texture,
            material_property,
        }
    }
}

impl <'n> Uniforms for CanvasUniform<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output : F) {
        if let Some(tex) = &self.font_texture {
            output("font_tex", tex.as_uniform_value());
        }
        for (name, value) in self.material_property.iter() {
            match value {
                PropertyValue::Bool(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Float(value) => output(name, value.as_uniform_value()),
                PropertyValue::Vec2(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Vec3(value)  => output(name, value.as_uniform_value()),
                PropertyValue::Vec4(value)  => output(name, value.as_uniform_value()),
            }
        }
    }
}