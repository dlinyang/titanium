use std::fs::{File,read_to_string};
use std::io::Write;
use serde_derive::{Serialize,Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetManager {
    pub meshes: HashMap<String,String>,
}

#[derive(Debug)]
pub enum AssetManagerFileErr {
    PathErr,
    SrcErr,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn open(path: &str) -> Result<Self,AssetManagerFileErr> {
        match read_to_string(path) {
            Ok(src) => match toml::from_str(src.as_str()) {
                Ok(result) => Ok(result),
                Err(_) => Err(AssetManagerFileErr::SrcErr),
            },
            Err(_) => Err(AssetManagerFileErr::PathErr),
        }
    }

    pub fn add_mesh(&mut self, name: String, path: String) {
        self.meshes.insert(name, path);
    }

    pub fn save(&self, path: &str) {
        let src = toml::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        let src_byte = src.as_bytes().clone();
        file.write_all(src_byte).unwrap();
        file.flush().unwrap();
    }
}