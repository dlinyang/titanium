use rmu::raw::Vec2f;
use crate::base::utils::*;
use super::ui::UIState;

pub struct W<R> where R: Canvas{
    pub view: dyn Fn(&R),
}

pub trait WidgetAction {
    fn id(&self) -> u64;
    fn update(&mut self, ui_state: &mut UIState) -> bool;
}

use crate::renderer::Canvas;

pub trait WidgetRender<R> where R: Canvas {
    fn render(&self, renderer: &mut R);
}

/// # example
/// ```ignore
/// let widget = Widget.new(..)
///         .attribute1(..)
///         .attribute2(..) 
///         ..
///         .build(..)
/// ```
pub trait WidgetBuilder: WidgetAction {
    fn new(name: &str) -> Self;
    fn build(self, ui_state: &mut UIState) -> Self;
}

/// This trait use for get WidgetAction trait and WidgetRender trait together
pub trait Widget<R>: WidgetAction +  WidgetRender<R> where  R: Canvas {

}

#[derive(Copy, Clone)]
pub enum WindowUnit<T> where T: Copy + Clone + Default{
    Pixel(T),
    Point(T),
}

impl<T> Default for WindowUnit<T> where T: Copy + Clone + Default {
    fn default() -> Self {
        WindowUnit::Pixel(Default::default())
    }
}

#[derive(Copy,Clone)]
pub enum Anchor {
    LeftTop(WindowUnit<Vec2f>),
    LeftMiddle(WindowUnit<Vec2f>),
    LeftBottom(WindowUnit<Vec2f>),
    RightTop(WindowUnit<Vec2f>),
    RightMiddle(WindowUnit<Vec2f>),
    RightBottom(WindowUnit<Vec2f>),
    TopMiddle(WindowUnit<Vec2f>),
    BottomMiddle(WindowUnit<Vec2f>),
    Center(WindowUnit<Vec2f>),
}

impl Anchor {
    pub fn value(&self) -> WindowUnit<Vec2f> {
        match &self {
            Anchor::LeftTop(w)      => *w,
            Anchor::LeftMiddle(w)   => *w,
            Anchor::LeftBottom(w)   => *w,
            Anchor::RightTop(w)     => *w,
            Anchor::RightMiddle(w)  => *w,
            Anchor::RightBottom(w)  => *w,
            Anchor::TopMiddle(w)    => *w,
            Anchor::Center(w)       => *w,
            Anchor::BottomMiddle(w) => *w,
        }
    }

    pub fn set(&mut self, value: WindowUnit<Vec2f>) {
        match &self {
            Anchor::LeftTop(_)      => *self = Anchor::LeftTop(value),
            Anchor::LeftMiddle(_)   => *self = Anchor::LeftMiddle(value),
            Anchor::LeftBottom(_)   => *self = Anchor::LeftBottom(value),
            Anchor::RightTop(_)     => *self = Anchor::RightTop(value),
            Anchor::RightMiddle(_)  => *self = Anchor::RightMiddle(value),
            Anchor::RightBottom(_)  => *self = Anchor::RightBottom(value),
            Anchor::TopMiddle(_)    => *self = Anchor::TopMiddle(value),
            Anchor::Center(_)       => *self = Anchor::Center(value),
            Anchor::BottomMiddle(_) => *self = Anchor::BottomMiddle(value),
        }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::LeftTop(WindowUnit::Point([0.0, 0.0]))
    }
}

#[warn(unused_assignments)]
pub fn area(anchor: Anchor, width: WindowUnit<f32>, height: WindowUnit<f32>, window_size: Size) -> Area {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut w: f32 = 0.0;
    let mut h: f32 = 0.0;

    match width {
        WindowUnit::Pixel(w0) => {
            w = w0 / window_size.width;
        },
        WindowUnit::Point(w0) =>{
            w = w0;
        },
    }

    match height {
        WindowUnit::Pixel(h0) => {
            h = h0 / window_size.height;
        },
        WindowUnit::Point(h0) => {
            h = h0;
        }
    }

    match anchor {

        Anchor::LeftTop(position) =>  match position {
            WindowUnit::Pixel([x0,y0]) => {
                x = x0 / window_size.width;
                y = y0 / window_size.height;
            },
            WindowUnit::Point([x0,y0]) => {
                x = x0;
                y = y0;
            },
        },

        Anchor::LeftMiddle(position) => match position {
            WindowUnit::Pixel([x0,y0]) => {
                x = x0 / window_size.width;
                y = y0 / window_size.height - h / 2.0;
            },
            WindowUnit::Point([x0,y0]) => {
                x = x0;
                y = y0 - h / 2.0;
            }
        }

        Anchor::LeftBottom(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width;
                y = y0 / window_size.height - h;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0;
                y = y0 - h;
            }
        },

        Anchor::RightTop(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width - w;
                y = y0 / window_size.height;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0 - w;
                y = y0;
            }
        },

        Anchor::RightMiddle(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width  - w;
                y = y0 / window_size.height - h / 2.0;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0 - w;
                y = y0 - h / 2.0;
            }
        },

        Anchor::RightBottom(position) =>  match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width - w;
                y = y0 / window_size.height - h;
            },
            WindowUnit::Point([x0, y0]) => {
               x = x0 - w;
               y = y0 - h; 
            }
        },

        Anchor::TopMiddle(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width - w / 2.0;
                y = y0 / window_size.height;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0 - w / 2.0;
                y = y0;
            }
        }

        Anchor::BottomMiddle(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width - w / 2.0;
                y = y0 / window_size.height - h;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0 - w / 2.0;
                y = y0 - h;
            }
        }

        Anchor::Center(position) => match position {
            WindowUnit::Pixel([x0, y0]) => {
                x = x0 / window_size.width - w / 2.0;
                y = y0 / window_size.height - h / 2.0;
            },
            WindowUnit::Point([x0, y0]) => {
                x = x0 - w / 2.0;
                y = y0 - h / 2.0;
            }
        },
    }

    Area {
        top_left_point: [x,y],
        bottom_right_point: [x + w, y + h],
    }
}