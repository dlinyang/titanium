use crate::event::*;
use glium::glutin;
use glutin::event::Event as GEvent;
use glutin::event::{WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use std::collections::VecDeque;

pub struct GLEventSystem {
    pub event_loop: EventLoop<()>,
    pub event_queue: VecDeque<Event>,
}

impl GLEventSystem {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        Self {
            event_loop,
            event_queue: VecDeque::new(),
        }
    }
}

impl EventSystem for GLEventSystem {

    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn event(&mut self) -> Event {

        use glutin::platform::desktop::EventLoopExtDesktop;
        if self.event_queue.is_empty() {
            let  mut event_queue = VecDeque::new();
            self.event_loop.run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                #[cfg(debug)]
                println!("{:?}",event);

                match event {
                    GEvent::NewEvents(_e) => (),
                    GEvent::WindowEvent{window_id: _, event} => match event {
                        WindowEvent::Resized(_physical_size) => (),
                        WindowEvent::Moved(_physical_position) => (),
                        WindowEvent::CloseRequested => event_queue.push_back(Event::Exit),
                        WindowEvent::Destroyed => (),
                        WindowEvent::DroppedFile(_path_buff) => (),
                        WindowEvent::HoveredFile(_path_buff) => (),
                        WindowEvent::HoveredFileCancelled => (),
                        WindowEvent::ReceivedCharacter(c) => event_queue.push_back(Event::Character(c)),
                        WindowEvent::Focused(_bool) => (),
                        WindowEvent::KeyboardInput {
                            device_id: _, 
                            input, 
                            is_synthetic: _
                        } => event_queue.push_back(match_keyboard(input)),
                        WindowEvent::ModifiersChanged(_modifiers_state) => (),
                        WindowEvent::CursorMoved {
                            device_id: _, 
                            position, 
                            modifiers: _,
                        } => event_queue.push_back(Event::CursorEvent{x: position.x as f32, y: position.y as f32}),
                        WindowEvent::CursorEntered { device_id: _ }  => (),
                        WindowEvent::CursorLeft { device_id: _} => (),
                        WindowEvent::MouseWheel {
                            device_id: _,
                            delta: _,
                            phase: _,
                            modifiers: _,
                        }  => (),
                        WindowEvent::MouseInput {
                            device_id: _,
                            state,
                            button,
                            modifiers: _,
                        } => event_queue.push_back(match_mouse_botton(state, button)),
                        WindowEvent::TouchpadPressure {
                            device_id: _,
                            pressure: _,
                            stage: _,
                        } => (),
                        WindowEvent::AxisMotion {
                            device_id: _,
                            axis: _,
                            value: _,
                        } => (),
                        WindowEvent::Touch(_touch) => (),
                        WindowEvent::ScaleFactorChanged {
                            scale_factor: _,
                            new_inner_size: _,
                        } => (),
                        WindowEvent::ThemeChanged(_theme) => (),
                    },
                    GEvent::DeviceEvent{device_id: _, event: _} => (),
                    GEvent::UserEvent(_e) => (),
                    GEvent::Suspended => (),
                    GEvent::Resumed => (),
                    GEvent::MainEventsCleared => *control_flow = ControlFlow::Exit,
                    GEvent::RedrawEventsCleared => (),
                    GEvent::RedrawRequested(_window_id) => (),
                    GEvent::LoopDestroyed => (),
                }
            });

        self.event_queue = event_queue;
        
        if self.event_queue.is_empty() {
            Event::Other
        } else {
            self.event_queue.pop_front().unwrap()
        }

    } else {
            self.event_queue.pop_front().unwrap()
        }
    }

    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn device_event(&mut self) -> Event {
        
        use glutin::platform::desktop::EventLoopExtDesktop;

        let mut result = Event::Other;

        result
    }
}

use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;

impl From<ElementState> for ButtonState {
    fn from(state: ElementState) -> ButtonState {
        match state {
            ElementState::Pressed => ButtonState::Press,
            ElementState::Released => ButtonState::Release,
        }
    } 
}

impl From<VirtualKeyCode> for Key {
    fn from(keycode: VirtualKeyCode) -> Key {
        match keycode {
            _ => Key::OTHER,
        }
    } 
}

fn match_keyboard(event: glutin::event::KeyboardInput) -> Event {
    let state = match event.state {
        ElementState::Pressed => ButtonState::Press,
        ElementState::Released => ButtonState::Release,
    };

    let mut key = Key::OTHER;
    if let Some(g_key) = event.virtual_keycode {
        match g_key {
            VirtualKeyCode::Key1 => key = Key::Num1,
            VirtualKeyCode::Key2 => key = Key::Num2,
            VirtualKeyCode::Key3 => key = Key::Num3,
            VirtualKeyCode::Key4 => key = Key::Num4,
            VirtualKeyCode::Key5 => key = Key::Num5,
            VirtualKeyCode::Key6 => key = Key::Num6,
            VirtualKeyCode::Key7 => key = Key::Num7,
            VirtualKeyCode::Key8 => key = Key::Num8,
            VirtualKeyCode::Key9 => key = Key::Num9,
            VirtualKeyCode::Key0 => key = Key::Num0,
            VirtualKeyCode::A => key = Key::A,
            VirtualKeyCode::B => key = Key::B,
            VirtualKeyCode::C => key = Key::C,
            VirtualKeyCode::D => key = Key::D,
            VirtualKeyCode::E => key = Key::E,
            VirtualKeyCode::F => key = Key::F,
            VirtualKeyCode::G => key = Key::G,
            VirtualKeyCode::H => key = Key::H,
            VirtualKeyCode::I => key = Key::I,
            VirtualKeyCode::J => key = Key::J,
            VirtualKeyCode::K => key = Key::K,
            VirtualKeyCode::L => key = Key::L,
            VirtualKeyCode::M => key = Key::M,
            VirtualKeyCode::N => key = Key::N,
            VirtualKeyCode::O => key = Key::O,
            VirtualKeyCode::P => key = Key::P,
            VirtualKeyCode::Q => key = Key::Q,
            VirtualKeyCode::R => key = Key::R,
            VirtualKeyCode::S => key = Key::S,
            VirtualKeyCode::T => key = Key::T,
            VirtualKeyCode::U => key = Key::U,
            VirtualKeyCode::V => key = Key::V,
            VirtualKeyCode::W => key = Key::W,
            VirtualKeyCode::X => key = Key::X,
            VirtualKeyCode::Y => key = Key::Y,
            VirtualKeyCode::Z => key = Key::Z,
            VirtualKeyCode::Up => key = Key::Up,
            VirtualKeyCode::Down => key = Key::Down,
            VirtualKeyCode::Left => key = Key::Left,
            VirtualKeyCode::Right => key = Key::Right,
            VirtualKeyCode::Delete => key = Key::Delete,
            VirtualKeyCode::Back => key = Key::Back,
            _=>(),
        };
    }

    Event::KeyEvent{state,key}
}

use glutin::event::MouseButton as GMouseButton;
fn match_mouse_botton(state: ElementState, button: GMouseButton) -> Event {
    let mouse_state = match state {
        ElementState::Pressed => ButtonState::Press,
        ElementState::Released => ButtonState::Release,
    };

    let mouse_button = match button {
        GMouseButton::Left => MouseButton::Left,
        GMouseButton::Middle => MouseButton::Middle,
        GMouseButton::Right => MouseButton::Right,
        GMouseButton::Other(_) => MouseButton::Other,
    };

    Event::MouseEvent {
        state: mouse_state,
        button: mouse_button,
    }
}