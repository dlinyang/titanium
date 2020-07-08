use rmu::raw::{Vec3f,Vec4f};

#[derive(Copy,Clone)]
pub struct Light {
    pub color_flux: Vec4f, //w is luminous flux
    pub position: Vec4f,
    pub direction_type: Vec4f, // w is light type
    // 0.0 is point light 1.0 is spot light 2.0 is parallel light
    pub cut_off: f32,
    pub outer_cut_off: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Light {
    /// create a default point light;
    pub fn new() -> Self {
        Self::point_light([1.0, 1.0, 1.0],[10.0,10.0,10.0])
    }

    pub fn point_light(color: Vec3f, position: Vec3f) -> Self {
        Self {
            color_flux: [color[0], color[1], color[2], 1.0],
            position: [position[0], position[1], position[2], 0.0],
            direction_type: [0.0, 0.0, 0.0, 0.0],
            cut_off: Default::default(),
            outer_cut_off: Default::default(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    pub fn parallel_light(color: Vec3f, position: Vec3f, direction: Vec3f) -> Self {
        Self {
            color_flux: [color[0], color[1], color[2], 1.0],
            position: [position[0], position[1], position[2], 0.0],
            direction_type: [direction[0],direction[1],direction[2],2.0],
            cut_off: Default::default(),
            outer_cut_off: Default::default(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    pub fn spot_light(color: Vec3f ,position: Vec3f, direction: Vec3f, phi: f32, dim: f32) -> Self {
        Self {
            color_flux: [color[0],color[1],color[2],1.0],
            position: [position[0],position[1],position[2],0.0],
            direction_type: [direction[0],direction[1],direction[2],1.0],
            cut_off: phi.cos(),
            outer_cut_off: (phi + dim).cos(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }
}