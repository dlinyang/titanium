use super::super::render_pass::*;

pub struct RenderPassSoure {
    pub name: String,
    pub source: GLSLSource,
    pub render_pass_type: RenderPassType,
    pub next: Option<Box<RenderPassSoure>>,
}

pub struct GLSLSource {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub geometry_shader: Option<String>,
}

pub trait ShaderCompiler<T> {
    fn compile(&self,source: RenderPassSoure) -> RenderPass<T>;
}

pub trait ShaderLoader {
    fn load(&mut self, soure: RenderPassSoure);
}