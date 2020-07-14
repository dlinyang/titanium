use crate::application::*;
use crate::renderer::*;
use crate::base::camera::Camera;
use crate::base::font::FontSet;

use glium::{Surface, Display, IndexBuffer, VertexBuffer,index, uniforms, buffer::*};
use std::collections::HashMap;

use super::buffer::*;
use super::font::*;
use super::pipeline::*;
use super::frame_data::FrameData;
use glium::Frame;

use std::rc::Rc;

pub struct GLRenderer {
    pub display           : Display,
    pub frame             : Option<Frame>,
    pub font_set          : FontSet,
    pub data_buffer       : DataBuffer,
    pub shader_buffer     : Rc<GLShaderBuffer>,
    pub frame_data        : FrameData,
    pub antialising_enable: bool,
    pub config            : Config,
    pub hdr_enable        : bool,
    pub gamma             : f32,
}

impl GLRenderer {
    pub fn new(config: Config, display: Display) -> Self {
        let mut shader_buffer = GLShaderBuffer::new();
        shader_buffer.load_bulidin(&display);

        Self {
            frame             : None,
            font_set          : FontSet::new(),
            data_buffer       : DataBuffer::new(&display),
            shader_buffer     : Rc::new(shader_buffer),
            frame_data        : FrameData::new(&display),
            antialising_enable: true,
            display,
            config,
            hdr_enable        : true,
            gamma             : 2.2,
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
        
        let scene_data = Rc::get_mut(&mut self.data_buffer.scene_data).unwrap();

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
                    let old_data = scene_data.data.insert(name.clone(), new_data);

                    // if material already exits
                    if let Some(same_material_data) = scene_data.same_material_data.get_mut(&data.material.name) {
                        same_material_data.insert(name.clone(),());
                    } else {
                        // material not exist
                        scene_data.same_material_data.insert(data.material.name.clone(), HashMap::new());
                        scene_data.same_material_data.get_mut(&data.material.name).unwrap().insert(name.clone(),());
                    }

                    if let Some(old_data) = old_data {
                        scene_data.same_material_data.get_mut(&old_data.material.name).unwrap().remove(&name.clone());
                    }
                },
                DataUpdate::Transfrom => {
                    if let Some(inner_data) = scene_data.data.get_mut(name) {
                        inner_data.transform = data.transform;
                    }
                },
                DataUpdate::Material => {
                    if let Some(inner_data) = scene_data.data.get_mut(name) {
                        inner_data.material = data.material.clone();
                    }
                },
                DataUpdate::Statue => {

                },
                DataUpdate::Not => (),
            }

            data.update = DataUpdate::Not;
        }
    }

    fn update_canvas(&mut self, canvas: &mut Canvas)  {

        let canvas_data = Rc::get_mut(&mut self.data_buffer.canvas_data).unwrap();

        /* check canvas change*/
        if canvas.id != canvas_data.id {
            canvas_data.clear();
            canvas_data.id = canvas.id;
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

                /* convert vertices to NDC*/
                let vertices: Vec<Position> = graphic
                    .positions
                    .iter()
                    .map(|v| { Position::new([2.0 * v.position[0] - 1.0, -2.0 * v.position[1] + 1.0], v.tex_coordinate)})
                    .collect();
                
                let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();

                render_layer.set_graphics(GraphicsData {
                    vertex_buffer,
                    indices: NoIndices(graphic.graphics_type.into()),
                    material: graphic.material.clone(),
                })
            }

            canvas_data.update(render_layer);
            
        }

    }

    fn update_camera(&mut self, camera: &Camera) {
        self.data_buffer.camera = camera.clone();
    }

    fn update_light(&mut self, name: &String, light: &Light) {
        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        light_buffer.lights.insert(name.clone(), light.clone());

        use glium::texture::*;

        let shadow_map_size = light_buffer.shadow_map_size;
        let shadow_map = DepthTexture2d::empty(&self.display, shadow_map_size, shadow_map_size).unwrap();
        light_buffer.shadow_maps.insert(name.clone(), shadow_map);

        let lights: Vec<Light> =light_buffer.lights.values().map(|x| {*x}).collect();
        light_buffer.buffer = Buffer::new(&self.display, lights.as_slice(), BufferType::UniformBuffer, BufferMode::default()).unwrap();
    }

    fn clear(&mut self) {
        let [r,g,b,a] = self.data_buffer.bg_color;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((r, g, b, a), 1.0);
        self.frame = Some(frame);
    }

    fn render(&mut self) {
        self.render_scene();
        self.render_canvas();
    }

    fn shadow_map(&mut self) {

        let mut camera = self.data_buffer.camera.clone();

        let mut shadow_map_views = Vec::new();

        for (name,light) in &self.data_buffer.light_buffer.lights {
            camera.set_look_from(light.position());

            use rmu::vector::Vector3;
            let project = if light.is_parallel(){
                camera.look_at = camera.look_from + Vector3::from(light.direction());
                camera.ortho()
            } else {
                camera.perspective()
            };

            let view = camera.view();

            let light_camera_matrix = uniforms::UniformBuffer::new(
                &self.display, 
                CameraMatrix {
                    project,
                    view: view,
                }
            ).unwrap();

            let shadow_map = self.data_buffer.light_buffer.shadow_maps.get(name).unwrap();

            use rmu::matrix::Matrix4x4;
            shadow_map_views.push((Matrix4x4::from(project) * Matrix4x4::from(view)).into());

            use glium::framebuffer::SimpleFrameBuffer;
            let mut frame = SimpleFrameBuffer::depth_only(&self.display, shadow_map).unwrap();

            frame.clear_color(1.0, 1.0, 1.0, 1.0);
            frame.clear_depth(1.0);

            use glium::draw_parameters::*;

            let paratmeter = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            use glium::uniform;

            for data in self.data_buffer.scene_data.data.values() {
                frame.draw(
                    &data.vertex_buffer, 
                    &data.indices, 
                    &self.frame_data.shadow_map, 
                    &uniform!{
                        Camera: &light_camera_matrix,
                        transform: data.transform,
                    }, 
                    &paratmeter,
                ).unwrap();
            }
        }

        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        light_buffer.shadow_map_views = shadow_map_views;
    }

    fn render_scene(&mut self) {
        let matrix = uniforms::UniformBuffer::new(
            &self.display, 
            CameraMatrix {
                project: self.data_buffer.camera.project(),
                view: self.data_buffer.camera.view(),
            }).unwrap();
        
        /* get lights buffer*/
        let light_buffer = self.data_buffer.light_buffer.clone();
        let lights_uniform = &light_buffer.unifrom_buffer();
        let lights_count = light_buffer.light_number() as i32;

        use glium::texture::DepthTexture2d;
        use glium::uniforms::*;

        let shadow_maps_ref: Vec<Sampler<'_,DepthTexture2d>> 
            = light_buffer.shadow_maps
                .values()
                .map( 
                    |x| Sampler::new(x)
                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
					        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                            .depth_texture_comparison(Some(DepthTextureComparison::LessOrEqual))
                )
                .collect();

        let shadow_map_views = light_buffer.shadow_map_views.clone();

        let mut uniform_data = SceneUniformData::new(
            &matrix, 
            &lights_uniform, 
            lights_count, 
            shadow_maps_ref, 
            shadow_map_views, 
            self.data_buffer.camera.look_from(), 
            self.hdr_enable, self.gamma
        );

        {
            let scene_data = self.data_buffer.scene_data.clone();
            let shader_buffer = self.shader_buffer.clone();

            for material_name in scene_data.same_material_data.keys() {
                if let Some(render_pass) = shader_buffer.shader(&material_name) {
                    self.scene_render_pass(&material_name, &mut uniform_data, render_pass);
                }
            }
        }

    }

    fn render_canvas(&mut self) {
        let mut layer_index: usize = 0;
        let canvas_data = self.data_buffer.canvas_data.clone();
        let shader_buffer = self.shader_buffer.clone();

        for render_layer in &canvas_data.data {
            if let Some(graphics) = &render_layer.graphics { 
                if let Some(pass) = shader_buffer.shader(&graphics.material.name) {
                    let mut uniform_data = CanvasUniformData::new(graphics.material.property());
                    self.canvas_render_pass(LayerIndex::Text(layer_index),&mut uniform_data,pass);
                }
            }

            if let Some(text) = &render_layer.text {
                if let Some(pass) = shader_buffer.shader(&text.material.name) {
                    let mut uniform_data = CanvasUniformData::new(text.material.property());
                    self.canvas_render_pass(LayerIndex::Graphics(layer_index),&mut uniform_data,pass);
                }
            }

            layer_index += 1;
        }
    }

    fn swap_buffer(&mut self)  {
        let target = self.frame.take().unwrap();

        target.finish().unwrap();
    }

    fn set_antialiasing(&mut self, enable: bool) {
        self.antialising_enable = enable;
    }

    fn set_hdr(&mut self, enable: bool) {
        self.hdr_enable = enable;
    }

    fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }
}