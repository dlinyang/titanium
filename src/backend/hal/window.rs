use winit::{
    dpi::LogicalSize, 
    Window,
    WindowBuilder,
    EventsLoop};

pub const WINDOW_NAME: &str = "Roaster";

#[derive(Debug)]
pub struct WindowState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

impl WindowState {
    fn new<T : Into<String>>( title: T, size: LogicalSize) -> Self {
        let event_loop = EventsLoop::new();
        let wb = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size);
        let window = wb.build(&event_loop).unwrap();
        
        WindowState {
            event_loop,
            window,
        }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self:: new(
            WINDOW_NAME,
            LogicalSize {
                width: 1024.0,
                height: 720.0,
            },
        )
    }
}