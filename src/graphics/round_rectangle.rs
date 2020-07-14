use rmu::raw::{Vec2f, Vec4f};
use crate::base::Position;
use crate::base::material::color_canvas;
use crate::renderer::canvas::graphics::{Graphics, GraphicsType};

#[derive(Clone,Copy)]
pub struct RoundRectangle {
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub round_angle: RoundAngle,
    pub color: Vec4f,
    pub graphics_type: GraphicsType,
}

impl RoundRectangle {
    pub fn new() -> Self {
        RoundRectangle {
            anchor: [0.9,0.9],
            w: 0.1,
            h: 0.05,
            round_angle: Default::default(),
            color: [0.1, 0.1, 0.5, 1.0],
            graphics_type: GraphicsType::PolygonFill,
        }
    }

    pub fn create(anchor: Vec2f, w: f32, h: f32, round_angle: RoundAngle, color: Vec4f, graphics_type: GraphicsType) -> Self {
        Self {
            anchor,
            w,
            h,
            round_angle,
            color,
            graphics_type,
        }
    }

    pub fn position(&self) -> Vec<Position> {
        self.round_angle.position(self.anchor[0], self.anchor[1], self.w, self.h)
    }
}

impl From<RoundRectangle> for Graphics {
    fn from(round_rectangle: RoundRectangle) -> Self {
        Self {
            positions: round_rectangle.position(),
            material: color_canvas(round_rectangle.color),
            graphics_type: round_rectangle.graphics_type,
        }
    }
}

#[derive(Copy,Clone)]
pub struct RoundAngle {
    pub top_left: Option<f32>,
    pub top_right: Option<f32>,
    pub bottom_right: Option<f32>,
    pub bottom_left: Option<f32>,
    pub sampling_times: f32,
}

impl Default for RoundAngle {
    fn default() -> Self {
        Self {
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
            sampling_times: 0.0,
        }
    }
}

impl RoundAngle {
    pub fn new(top_left: Option<f32>, top_right: Option<f32>, bottom_right: Option<f32>, bottom_left: Option<f32>) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            sampling_times: 11.0,
        }
    }

    pub fn position(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<Position> {

        let mut result = Vec::new();

        use std::f32::consts::FRAC_PI_4;

        if let Some(radius) = &self.top_left {
            let mut i = 0.0;
            while i <= self.sampling_times {
                result.push(
                    Position::from([
                        x + radius - radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + radius - radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                        ])
                );
                i += 1.0;
            }
        } else {
            result.push(Position::from([x,y]));
        }

        if let Some(radius) = &self.top_right {
            let mut i = self.sampling_times;
            while i >= 0.0 {
                result.push(
                    Position::from([
                        x + w - radius + radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + radius - radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                        ])
                );
                i -= 1.0;
            }
        } else {
            result.push(Position::from([x + w, y]));
        }

        if let Some(radius) = &self.bottom_right {
            let mut i = 0.0;
            while i <= self.sampling_times {
                result.push(
                    Position::from([
                        x + w - radius + radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + h - radius + radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                        ])
                );
                i += 1.0;
            }
        } else {
            result.push(Position::from([x + w, y + h]));
        }

        if let Some(radius) = &self.bottom_left {
            let mut i = self.sampling_times;
            while i >= 0.0 {
                result.push(
                    Position::from([
                        x + radius - radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + h - radius + radius * (FRAC_PI_4 * i  / self.sampling_times).sin()
                        ])
                );
                i -= 1.0;
            }
        } else {
            result.push(Position::from([x, y + h]));
        }

        result
    }
}