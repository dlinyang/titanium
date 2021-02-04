use super::renderer::GLRenderer;
use super::pipeline::uniforms::*;
use crate::renderer::{
    RenderProdure, 
    RenderPassRenderer, 
    pipeline::ShaderBuffer,
};
use glium::uniforms::UniformBuffer;
use glium::Surface;
use std::rc::Rc;

impl RenderProdure for GLRenderer {
    fn clear(&mut self) {
        let [r,g,b,a] = self.data_buffer.bg_color;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((r, g, b, a), 1.0);
        self.frame = Some(frame);
    }

    fn shadow_map(&mut self) {

        let mut camera = self.data_buffer.camera.clone();

        // let mut shadow_map_views = Vec::new();

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

            let light_camera_matrix = UniformBuffer::new(
                &self.display, 
                CameraMatrix {
                    project,
                    view: view,
                }
            ).unwrap();

            let shadow_map = self.data_buffer.light_buffer.shadow_maps.get(name).unwrap();

            //use rmu::matrix::Matrix4x4;
            //shadow_map_views.push((Matrix4x4::from(project) * Matrix4x4::from(view)).into());

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

            for object in self.data_buffer.scene_buffer.objects.values() {
                if let Some(mesh) = self.data_buffer.scene_buffer.meshes.get(&object.mesh_name) {
                    frame.draw(
                        &mesh.vertex_buffer, 
                        &mesh.index_buffer, 
                        &self.screen_data.shadow_map, 
                        &uniform!{
                            Camera: &light_camera_matrix,
                            transform: object.transform,
                        }, 
                        &paratmeter,
                        ).unwrap();
                }
            }
        }

        let light_buffer = Rc::get_mut(&mut self.data_buffer.light_buffer).unwrap();
        // light_buffer.shadow_map_views = shadow_map_views;
    }

    fn render(&mut self) {
        let matrix = UniformBuffer::new(
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

        // let shadow_map_views = light_buffer.shadow_map_views.clone();

        let mut uniform_data = SceneUniformData::new(
            &matrix, 
            &lights_uniform, 
            lights_count, 
            self.data_buffer.camera.look_from(), 
            self.hdr_enable, self.gamma
        );

        {
            let scene_data = self.data_buffer.scene_buffer.clone();
            let shader_buffer = self.shader_buffer.clone();

            for material_name in scene_data.same_material_objects.keys() {
                let material = scene_data.materials.get(material_name).unwrap();
                if let Some(render_pass) = shader_buffer.shader(&material.name) {
                    self.render_pass(&material_name, &mut uniform_data, render_pass);
                }
            }
        }

    }

    fn swap_buffer(&mut self)  {
        if let Some(target) = self.frame.take() {
            target.finish().unwrap();
        }
    }
}