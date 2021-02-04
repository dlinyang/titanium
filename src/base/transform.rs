use rmu::raw::Vec3f;
use rmu::matrix::Matrix4x4;

#[derive(Copy,Clone)]
pub struct Transform {
    pub rotation: Vec3f,
    pub location: Vec3f,
    pub scale: Vec3f,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: [0.0,0.0,0.0],
            location: [0.0,0.0,0.0],
            scale: [1.0,1.0,1.0],
        }
    }

    pub fn transform(&self) -> Matrix4x4 {
        let position = Self::position(self.location[0], self.location[1], self.location[2]);
        let rotation = Self::rotation(self.rotation[0], self.rotation[1], self.rotation[2]);
        let scale = Self::scale(self.scale[0], self.scale[1], self.scale[2]);

        position * rotation * scale
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.location[0] += x;
        self.location[1] += y;
        self.location[2] += z;
    }

    pub fn add_rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation[0] += x;
        self.rotation[1] += y;
        self.rotation[2] += z;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [x,y,z];
    }

    /// left multiplicative matrix
    pub fn position(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::from(
            [[1.0, 0.0, 0.0, 0.0]
            ,[0.0, 1.0, 0.0, 0.0]
            ,[0.0, 0.0, 1.0, 0.0]
            ,[ x ,  y ,  z , 1.0]])
    }

    pub fn rotation(x: f32, y: f32,z: f32) -> Matrix4x4 {
        Matrix4x4::from(
            [[z.cos()*x.cos()-y.cos()*x.sin()*z.sin(), -z.sin()*y.sin()*x.sin() - x.cos()*z.sin(), x.sin()*y.sin() , 0.0]
            ,[z.cos()*x.sin()+x.cos()*y.cos()*z.sin(), x.cos()*y.cos()*z.cos() - x.sin()*z.cos() , -x.cos()*y.sin(), 0.0]
            ,[         y.sin()*z.cos()               ,                z.sin()*y.cos()            ,        y.cos()  , 0.0]
            ,[                   0.0                 ,                      0.0                  ,         0.0     , 1.0]]
        )
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::from(
            [[ x , 0.0, 0.0, 0.0]
            ,[0.0,  y , 0.0, 0.0]
            ,[0.0, 0.0,  z , 0.0]
            ,[0.0, 0.0, 0.0, 1.0]]
        )
    }

}

use rmu::raw::Mat4f;

impl From<Transform> for Mat4f {
    fn from(transform: Transform) -> Self {
        transform.transform().into()
    }
}

use rmu::raw::Vec2f;
use rmu::matrix::Matrix3x3;

#[derive(Copy,Clone)]
pub struct Transform2D {
    pub rotation: f32,
    pub position: Vec2f,
    pub scale: Vec2f,
}

impl Transform2D {
    pub fn new() -> Self {
        Self {
            rotation: 0.0,
            position: [0.0,0.0],
            scale: [1.0,1.0],
        }
    }

    pub fn transform(&self) -> Matrix3x3 {
        let position = Self::position(self.position[0], self.position[1]);
        let rotation = Self::rotation(self.rotation);
        let scale = Self::scale(self.scale[0], self.scale[1]);

        position * rotation * scale
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position[0] += x;
        self.position[1] += y;
    }

    pub fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = [x,y];
    }

    /// left multiplicative matrix
    pub fn position(x: f32, y: f32) -> Matrix3x3 {
        Matrix3x3::from(
            [[1.0, 0.0, 0.0]
            ,[0.0, 1.0, 0.0]
            ,[ x ,  y , 1.0]])
    }

    pub fn rotation(theta: f32) -> Matrix3x3 {
        Matrix3x3::from(
            [[theta.cos(), -theta.sin(), 0.0]
            ,[theta.sin(), -theta.cos(), 0.0]
            ,[   0.0     ,     0.0     , 1.0]])
    }

    pub fn scale(x: f32, y: f32) -> Matrix3x3 {
        Matrix3x3::from(
            [[ x , 0.0, 0.0]
            ,[0.0,  y , 0.0]
            ,[0.0, 0.0, 1.0]]
        )
    }

}
