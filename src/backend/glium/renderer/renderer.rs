use crate::application::*;
use crate::renderer::*;
use crate::base::camera::Camera;
use crate::base::font::FontSet;

use glium::{Surface, Display, IndexBuffer, VertexBuffer,index, uniforms, buffer::*};
use std::collections::HashMap;

use super::buffer::*;
use super::pipeline::*;
use super::screen_data::*;
use glium::Frame;

use std::rc::Rc;

pub struct GLRenderer {
    pub display           : Display,
    pub frame             : Option<Frame>,
    pub font_set          : FontSet,
    pub data_buffer       : DataBuffer,
    pub shader_buffer     : Rc<GLShaderBuffer>,
    pub screen_data       : ScreenData,
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
            screen_data       : ScreenData::new(&display),
            antialising_enable: true,
            display,
            config,
            hdr_enable        : true,
            gamma             : 2.2,
        }
    }
}

use rmu::raw::Vec4f;
use crate::base::Index;
use scene::DataUpdate;

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

    fn update_texture(&mut self, data: ImageData, name: &String) {
        use std::borrow::Cow;

        let texture = Texture2d::with_format(
            &self.display,
            RawImage2d {
                data: Cow::Owned(data.data),
                width: data.dimensions.0,
                height: data.dimensions.1,
                format: data.image_type.into(),
            },
            data.image_type.into(),
            MipmapsOption::NoMipmap,
        ).unwrap();

        self.data_buffer.texture_buffer.insert(name.clone(), texture);

    }

    fn remove_texture(&mut self, name: &String) {
        self.data_buffer.texture_buffer.remove(name);
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

    fn remove_light(&mut self, name: &String) {
        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        if let Some(_) = light_buffer.lights.remove(name) {
            light_buffer.shadow_maps.remove(name);
            let lights: Vec<Light> =light_buffer.lights.values().map(|x| {*x}).collect();
            light_buffer.buffer = Buffer::new(&self.display, lights.as_slice(), BufferType::UniformBuffer, BufferMode::default()).unwrap();
        }
    }

    fn clear(&mut self) {
        let [r,g,b,a] = self.data_buffer.bg_color;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((r, g, b, a), 1.0);
        self.frame = Some(frame);
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
                    &self.screen_data.shadow_map, 
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

    fn render(&mut self) {
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
                    self.render_pass(&material_name, &mut uniform_data, render_pass);
                }
            }
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

use crate::renderer::image::*;
use glium::texture::*;

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