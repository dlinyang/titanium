use rmu::raw::Vec2f;
use super::Graphics2d;

#[derive(Clone,Copy)]
pub struct RoundRectangle {
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub round_angle: RoundAngle,
}

impl RoundRectangle {
    pub fn new() -> Self {
        RoundRectangle {
            anchor: [0.9,0.9],
            w: 0.1,
            h: 0.05,
            round_angle: Default::default(),
        }
    }

    pub fn create(anchor: Vec2f, w: f32, h: f32, round_angle: RoundAngle) -> Self {
        Self {
            anchor,
            w,
            h,
            round_angle,
        }
    }
}

impl Graphics2d for RoundRectangle {
    fn positions(&self) -> Vec<Vec2f> {
        self.round_angle.position(self.anchor[0], self.anchor[1], self.w, self.h)
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

    pub fn uniform(value: f32) -> Self {
        Self {
            top_left: Some(value),
            top_right: Some(value),
            bottom_left: Some(value),
            bottom_right: Some(value),
            sampling_times: 11.0,
        }
    }

    pub fn div(&mut self, value: f32) {
        if let Some(r) = &self.bottom_left {
            self.bottom_left = Some( r / value );
        }

        if let Some(r) = &self.bottom_right {
            self.bottom_right = Some( r / value );
        }

        if let Some(r) = &self.top_left {
            self.top_left = Some( r / value );
        }

        if let Some(r) = &self.top_right {
            self.top_right = Some( r / value );
        }
    }

    pub fn position(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<Vec2f> {

        let mut result = Vec::new();

        use std::f32::consts::FRAC_PI_4;

        if let Some(radius) = &self.top_left {
            let mut i = 0.0;
            while i <= self.sampling_times {
                result.push(
                    [
                        x + radius - radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + radius - radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                        ]
                );
                i += 1.0;
            }
        } else {
            result.push([x,y]);
        }

        if let Some(radius) = &self.top_right {
            let mut i = self.sampling_times;
            while i >= 0.0 {
                result.push(
                    [
                        x + w - radius + radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + radius - radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                        ]
                );
                i -= 1.0;
            }
        } else {
            result.push([x + w, y]);
        }

        if let Some(radius) = &self.bottom_right {
            let mut i = 0.0;
            while i <= self.sampling_times {
                result.push(
                    [
                        x + w - radius + radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + h - radius + radius * (FRAC_PI_4 * i / self.sampling_times).sin()
                       ]
                );
                i += 1.0;
            }
        } else {
            result.push([x + w, y + h]);
        }

        if let Some(radius) = &self.bottom_left {
            let mut i = self.sampling_times;
            while i >= 0.0 {
                result.push(
                    [
                        x + radius - radius * (FRAC_PI_4 * i / self.sampling_times).cos(),
                        y + h - radius + radius * (FRAC_PI_4 * i  / self.sampling_times).sin()
                        ]
                );
                i -= 1.0;
            }
        } else {
            result.push([x, y + h]);
        }

        result
    }
}