pub struct GLSLSource {
    pub name: String,
    pub vertex_shader: GLSLStage,
    pub fragment_shader: GLSLStage,
    pub geometry_shader: Option<GLSLStage>,
}

pub struct GLSLStage {
    pub include: String,
    pub src: String,
}