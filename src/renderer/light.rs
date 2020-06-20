use rmu::raw::Vec3f;

#[derive(Copy,Clone)]
pub struct Light {
    pub is_position: bool,
    pub is_range: bool,
    pub color: Vec3f,
    pub position: Vec3f,
    pub direction: Vec3f,
    pub cut_off: f32,
    pub outer_cut_off: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Light {
    /// create a point light;
    pub fn new() -> Self {
        Self::point_light([1.0, 1.0, 1.0],[10.0,10.0,10.0])
    }

    pub fn point_light(color: Vec3f, position: Vec3f) -> Self {
        Self {
            is_position: true,
            is_range: false,
            color,
            position,
            direction: Default::default(),
            cut_off: Default::default(),
            outer_cut_off: Default::default(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    pub fn parallel_light(color: Vec3f, direction: Vec3f) -> Self {
        Self {
            is_position: false,
            is_range: false,
            color,
            position: Default::default(),
            direction,
            cut_off: Default::default(),
            outer_cut_off: Default::default(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    pub fn spot_light(color: Vec3f ,position: Vec3f, direction: Vec3f, phi: f32, dim: f32) -> Self {
        Self {
            is_position: true,
            is_range: true,
            color,
            position,
            direction,
            cut_off: phi.cos(),
            outer_cut_off: (phi + dim).cos(),
            linear: 0.09,
            quadratic: 0.032,
        }
    }
}