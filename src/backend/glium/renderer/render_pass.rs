use super::renderer::*;
use super::pipeline::*;
use glium::program::Program;
use glium::{draw_parameters::*,Surface};
use glium::framebuffer::{SimpleFrameBuffer,MultiOutputFrameBuffer};
use glium::texture::*;
use glium::texture::Texture2d;
use glium::texture::depth_texture2d::DepthTexture2d;
use crate::renderer::RenderPassRenderer;
use crate::renderer::pipeline::*;

impl RenderPassRenderer<SceneUniformData<'_>,Program> for GLRenderer {
    fn render_pass(&mut self, material_name: &String, uniform_data: &mut SceneUniformData, render_pass: &RenderPass<Program>) {
        let mut parameters: DrawParameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            multisampling: self.antialising_enable,
            ..Default::default()
        };

        parameters.load(&render_pass.pass_option);

        match &render_pass.render_pass_type {
            RenderPassType::Pass => {
                if let Some(frame) = &mut self.frame {
                    if let Some(same_material_data) = self.data_buffer.scene_data.same_material_data.get(material_name) {
                        for data_name in same_material_data.keys() {
                            let data = self.data_buffer.scene_data.data.get(data_name).unwrap();
                   
                            let uniforms = SceneUniform::new(
                                uniform_data, 
                                data.transform, 
                                data.material.property(), 
                                render_pass.pass_option.lighting, 
                                &self.data_buffer.texture_buffer
                            );
                   
                            frame.draw(&data.vertex_buffer, &data.indices, &render_pass.shader, &uniforms, &parameters).unwrap();
                        }
                    }
                }
            },
            RenderPassType::TargetPass((name,format)) => {

                let w = self.config.size.width as u32;
                let h = self.config.size.height as u32;

                /* get render texture reference */
                let texture_ref = if let Some(texture_ref) = self.data_buffer.texture_buffer.get(name) {
                    texture_ref
                } else {
                    let tex = Texture2d::empty_with_format(
                        &self.display,
                        From::from(*format),
                        MipmapsOption::NoMipmap,
                        w,
                        h
                    ).unwrap();
                    self.data_buffer.texture_buffer.insert(name.clone(),tex);
                    self.data_buffer.texture_buffer.get(name).unwrap()
                };

                /* get depth texture refernece */
                let depth_ref = if let Some(depth_ref) = self.data_buffer.depth_texture.as_ref() {
                    depth_ref
                } else {
                    let depth = DepthTexture2d::empty_with_format(
                        &self.display,
                        DepthFormat::I24,
                        MipmapsOption::NoMipmap,
                        w,
                        h
                    ).unwrap();
                    self.data_buffer.depth_texture = Some(depth);
                    self.data_buffer.depth_texture.as_ref().unwrap()
                };

                let mut frame = SimpleFrameBuffer::with_depth_buffer(&self.display, texture_ref, depth_ref).unwrap();

                if let Some(same_material_data) = self.data_buffer.scene_data.same_material_data.get(material_name) {
                    for data_name in same_material_data.keys() {
                        let data = self.data_buffer.scene_data.data.get(data_name).unwrap();

                        let uniforms = SceneUniform::new(
                            uniform_data, data.transform, 
                            data.material.property(), 
                            render_pass.pass_option.lighting, 
                            &self.data_buffer.texture_buffer
                        );
            
                        frame.draw(&data.vertex_buffer, &data.indices, &render_pass.shader, &uniforms, &parameters).unwrap();
                    }
                }

                uniform_data.render_target = Some(name.clone());
            },
            RenderPassType::MultipleRenderPass(output) => {

                let mut multiple_render_target = Vec::new();
                let mut frame_output: Vec<(&str,&Texture2d)> =  Vec::new();

                let w = self.config.size.width as u32;
                let h = self.config.size.height as u32;

                /* if texture not exit then generate and insert*/
                for (name,format) in output.iter() {
                    if let None = self.data_buffer.texture_buffer.get(name) {
                        let tex = Texture2d::empty_with_format(
                            &self.display,
                            From::from(*format),
                            glium::texture::MipmapsOption::NoMipmap,
                            w,
                            h,
                        ).unwrap();
                        self.data_buffer.texture_buffer.insert(name.clone(), tex);
                    }
                }

                /*get frame output textures referrence*/
                for (name,_) in output.iter() {
                    let tex = self.data_buffer.texture_buffer.get(name).unwrap();
                    multiple_render_target.push(name.clone());
                    frame_output.push((name.as_str(),tex));
                }

                /* get depth reference */
                let depth_ref = if let Some(depth_ref) = self.data_buffer.depth_texture.as_ref() {
                    depth_ref
                } else {
                    let depth = DepthTexture2d::empty_with_format(
                        &self.display,
                        DepthFormat::I24,
                        MipmapsOption::NoMipmap,
                        w,
                        h
                    ).unwrap();
                    self.data_buffer.depth_texture = Some(depth);
                    self.data_buffer.depth_texture.as_ref().unwrap()
                };

                let mut frame = MultiOutputFrameBuffer::with_depth_buffer(&self.display, frame_output, depth_ref).unwrap();

                if let Some(same_material_data) = self.data_buffer.scene_data.same_material_data.get(material_name) {
                    for data_name in same_material_data.keys() {
                        let data = self.data_buffer.scene_data.data.get(data_name).unwrap();

                        let uniforms = SceneUniform::new(
                            uniform_data, 
                            data.transform, 
                            data.material.property(), 
                            render_pass.pass_option.lighting, 
                            &self.data_buffer.texture_buffer
                        );

                        frame.draw(&data.vertex_buffer, &data.indices, &render_pass.shader, &uniforms, &parameters).unwrap();
                    }
                }

                uniform_data.multiple_render_target = multiple_render_target;
            },
            RenderPassType::PostPressingPass => {
                if let Some(frame) = &mut self.frame {

                    let uniforms = SceneUniform::new(
                        uniform_data, 
                        Default::default(), 
                        Vec::new(), 
                        render_pass.pass_option.lighting, 
                        &self.data_buffer.texture_buffer
                    );

                    frame.draw(
                        &self.screen_data.vertex_buffer, 
                        &self.screen_data.index_buffer, 
                        &render_pass.shader, 
                        &uniforms, 
                        &parameters
                    ).unwrap();
                }
            }
            RenderPassType::TargetPostPressingPass((name,format)) => {
                    let w = self.config.size.width as u32;
                    let h = self.config.size.height as u32;

                    /* get render texture reference*/
                    let texture_ref = if let Some(texture_ref) = self.data_buffer.texture_buffer.get(name) {
                        texture_ref
                    } else {
                        let tex = Texture2d::empty_with_format(
                            &self.display,
                            From::from(*format),
                            MipmapsOption::NoMipmap,
                            w,
                            h
                        ).unwrap();
                        self.data_buffer.texture_buffer.insert(name.clone(),tex);
                        self.data_buffer.texture_buffer.get(name).unwrap()
                    };

                    /* get depth texture  reference */
                    let depth_ref = if let Some(depth_ref) = self.data_buffer.depth_texture.as_ref() {
                        depth_ref
                    } else {
                        let depth = DepthTexture2d::empty_with_format(
                            &self.display,
                            DepthFormat::I24,
                            MipmapsOption::NoMipmap,
                            w,
                            h
                        ).unwrap();
                        self.data_buffer.depth_texture = Some(depth);
                        self.data_buffer.depth_texture.as_ref().unwrap()
                    };

                    let mut frame = SimpleFrameBuffer::with_depth_buffer(&self.display, texture_ref, depth_ref).unwrap();

                    let uniforms = SceneUniform::new(
                        uniform_data, 
                        Default::default(), 
                        Vec::new(), 
                        render_pass.pass_option.lighting, 
                        &self.data_buffer.texture_buffer
                    );
                   
                    frame.draw(
                        &self.screen_data.vertex_buffer, 
                        &self.screen_data.index_buffer, 
                        &render_pass.shader, 
                        &uniforms, 
                        &parameters
                    ).unwrap();

                    uniform_data.render_target = Some(name.clone());
            }
        }

        /* next render pass*/
        match &render_pass.next {
            Some(next_pass) => self.render_pass(material_name, uniform_data, next_pass),
            None => (),
        };
    }
}

impl From<OutputFormat> for UncompressedFloatFormat {
    fn from(output: OutputFormat) -> Self {
        match output {
            OutputFormat::U8 => UncompressedFloatFormat::U8,
            OutputFormat::Vec2U8 => UncompressedFloatFormat::U8U8,
            OutputFormat::Vec3U8 => UncompressedFloatFormat::U8U8U8,
            OutputFormat::Vec4U8 => UncompressedFloatFormat::U8U8U8U8,
            OutputFormat::F16 => UncompressedFloatFormat::F16,
            OutputFormat::Vec2F16 => UncompressedFloatFormat::F16F16,
            OutputFormat::Vec3F16 => UncompressedFloatFormat::F16F16F16,
            OutputFormat::Vec4F16 => UncompressedFloatFormat::F16F16F16F16,
            OutputFormat::F32 => UncompressedFloatFormat::F32,
            OutputFormat::Vec2 => UncompressedFloatFormat::F32F32,
            OutputFormat::Vec3 => UncompressedFloatFormat::F32F32F32,
            OutputFormat::Vec4 => UncompressedFloatFormat::F32F32F32F32,
        }
    }
}

impl From<FaceCulling> for BackfaceCullingMode {
    fn from(culling: FaceCulling) -> Self {
        match culling {
            FaceCulling::Not   => BackfaceCullingMode::CullingDisabled,
            FaceCulling::Front => BackfaceCullingMode::CullClockwise,
            FaceCulling::Back  => BackfaceCullingMode::CullCounterClockwise,
        }
    }
}

impl From<ZTest> for DepthTest {
    fn from(test: ZTest) -> Self {
        match test {
            ZTest::Less           => DepthTest::IfLess,
            ZTest::LessOrEqual    => DepthTest::IfLessOrEqual,
            ZTest::Greater        => DepthTest::IfMore,
            ZTest::GreaterOrEqual => DepthTest::IfMoreOrEqual,
            ZTest::Equal          => DepthTest::IfEqual,
            ZTest::NotEqual       => DepthTest::IfNotEqual,
            ZTest::Off            => DepthTest::Ignore,
            ZTest::Always         => DepthTest::Overwrite,
        }
    }
}

impl PassOptionLoader for DrawParameters<'_> {
    fn load(&mut self, pass_option: &PassOption) {
        self.point_size = pass_option.point_size;
        self.line_width = pass_option.line_width;
        self.depth.test = From::from(pass_option.z_test);
        self.depth.write = pass_option.z_write;
    }
}