use std::collections::HashMap;
use super::RenderData;

pub struct RenderScene {
    pub render_data: HashMap<String,RenderData>,
}

impl RenderScene {
    pub fn new() -> Self {
        Self {
            render_data: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, render_data: RenderData) {
        self.render_data.insert(name, render_data);
    }
}