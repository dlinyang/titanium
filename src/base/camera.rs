use rmu::raw::{Vec3f,Mat4f};
use rmu::geometry::transform::rotation3;

#[derive(Debug,Copy,Clone)]
pub struct Camera {
    pub look_from: Vector3,
    pub look_at: Vector3,
    // view up
    pub vup: Vector3,
    // field of view
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect_radio: f32,
}

impl Camera {
    pub fn new(look_from: Vec3f, look_at: Vec3f, vup: Vec3f, aspect_radio: f32) -> Self {
        Self {
            look_from: look_from.into(),
            look_at: look_at.into(),
            vup: vup.into(),
            fov: std::f32::consts::PI / 3.0,
            near: 0.1,
            far: 1024.0,
            aspect_radio,
        }
    }

    #[inline]
    pub fn view(&self) -> Mat4f {
        camera(self.look_from, self.look_at, self.vup)
    }

    #[inline]
    pub fn look_from(&self) -> Vec3f {
        [self.look_from[0], self.look_from[1], self.look_from[2]]
    }

    #[inline]
    pub fn look_at(&self) -> Vec3f {
        [self.look_at[0], self.look_at[1], self.look_at[2]]
    }

    pub fn translation(&mut self, x: f32, y: f32, z: f32) {
        let x_axis = (self.look_at - self.look_from).normalized();
        let y_axis = Vector3::cross(self.vup, x_axis).normalized();
        let z_axis = Vector3::cross(y_axis, x_axis).normalized();

        let x_axis_translation = x * x_axis;
        let y_axis_translation = y * y_axis;
        let z_axis_translation = z * z_axis;

        self.look_at = self.look_at + x_axis_translation + y_axis_translation + z_axis_translation;
        self.look_from = self.look_from + x_axis_translation + y_axis_translation + z_axis_translation;
    }

    pub fn rotation(&mut self, x: f32, y: f32, z: f32)  {
        let rotate_matrix = rotation3(x, y, z);
        let direction = self.look_at - self.look_from;
        let new_direction = direction * rotate_matrix;
        self.look_at = self.look_from + new_direction;
    }

    pub fn rotation_with_look_at(&mut self, x: f32, y: f32, z: f32) {
        let rotate_matrix = rotation3(x, y, z);
        let direction = self.look_from - self.look_at;
        let new_direction = direction * rotate_matrix;
        self.look_from = self.look_at + new_direction;
    }

    pub fn perspective(&self) -> Mat4f {
        perspective(self.fov, self.far, self.near, self.aspect_radio)
    }

    pub fn ortho(&self) -> Mat4f {
        ortho(self.fov, self.far, self.near, self.aspect_radio)
    }
}

use rmu::vector::Vector3;

pub fn camera(look_from: Vector3, look_at: Vector3, vup: Vector3) -> Mat4f {
    let w = (look_from - look_at).normalized();
    let u = Vector3::cross(Vector3::from(vup), w).normalized();
    let v = Vector3::cross(w, u);

    let p0 = Vector3::dot(-look_from, u);
    let p1 = Vector3::dot(-look_from, v);
    let p2 = Vector3::dot(-look_from, w);

    [[u[0], v[0], w[0], 0.0]
    ,[u[1], v[1], w[1], 0.0]
    ,[u[2], v[2], w[2], 0.0]
    ,[p0  , p1  , p2  , 1.0]]
}

/// perspective transfrom is non-linear transform.
/// orthogonal transform is linear transomt.
/// martix transform is linear trnsform,
/// matrix can not impletementperspective tansform. 
/// but graphic api will divide x,y,z by w    
 pub fn perspective(fov: f32, far: f32, near: f32, aspect_radio: f32) -> Mat4f {
     let f = 1.0/(fov/2.0).tan();
     [[f * aspect_radio , 0.0,            0.0           ,  0.0]
     ,[      0.0        , f  ,            0.0           ,  0.0]
     ,[      0.0        , 0.0, (far+near)/(near - far)  , -1.0]
     ,[      0.0        , 0.0, 2.0*far*near/(near - far),  0.0]]
 }

pub fn ortho(fov: f32, far: f32, near: f32, aspect_radio: f32) -> Mat4f{
    let f = 1.0/(fov/2.0).tan();
    [[ f * aspect_radio, 0.0,       0.0       , 0.0]
    ,[      0.0        ,  f ,       0.0       , 0.0]
    ,[      0.0        , 0.0, 2.0/(near - far), 0.0]
    ,[      0.0        , 0.0,       0.0       , 1.0]]
}