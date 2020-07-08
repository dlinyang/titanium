pub enum RenderPassType {
    Pass,
    TargetPass((String,OutputFormat)),
    MultipleRenderPass(Vec<(String,OutputFormat)>),
    PostPressingPass,
    TargetPostPressingPass((String,OutputFormat)),
}

pub struct RenderPass<T> {
    pub shader: T,
    pub pass_option: PassOption,
    pub render_pass_type: RenderPassType,
    pub next: Option<Box<RenderPass<T>>>
}

impl<T> RenderPass<T> {
    pub fn pass(shader: T,  next: Option<Box<RenderPass<T>>>) -> Self {
        Self {
            shader,
            render_pass_type: RenderPassType::Pass,
            pass_option: Default::default(),
            next,
        }
    }
    
    pub fn multiple_render_pass(shader: T, output: Vec<(String,OutputFormat)>, next: Box<RenderPass<T>>) -> Self {
        Self {
                shader,
                render_pass_type: RenderPassType::MultipleRenderPass(output),
                pass_option: Default::default(),
                next: Some(next),
        }
    }

    pub fn with_depth(mut self, z_test: ZTest, z_write: bool) -> Self {
        self.pass_option.z_test = z_test;
        self.pass_option.z_write = z_write;
        self
    }
}

#[derive(Copy,Clone)]
pub enum OutputFormat {
    U8,
    Vec2U8,
    Vec3U8,
    Vec4U8,
    F16,
    Vec2F16,
    Vec3F16,
    Vec4F16,
    F32,
    Vec2,
    Vec3,
    Vec4,
}

pub struct PassOption {
    pub point_size: Option<f32>,
    pub line_width: Option<f32>,
    pub lighting: bool,
    pub face_culling: FaceCulling,
    pub z_test: ZTest,
    pub z_write: bool,
}

impl Default for PassOption {
    fn default() -> Self {
        Self {
            point_size: None,
            line_width: None,
            lighting: true,
            face_culling: FaceCulling::Not,
            z_test: ZTest::LessOrEqual,
            z_write: true,
        }
    }
}

#[derive(Copy,Clone)]
pub enum FaceCulling {
    Not,
    Front,
    Back,
}

#[derive(Copy,Clone)]
pub enum ZTest {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Off,
    Always,
}

pub trait PassOptionLoader {
    fn load(&mut self, pass_option: &PassOption);  
}