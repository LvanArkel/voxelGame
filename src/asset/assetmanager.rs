use std::{collections::HashMap, path::Path};

use image::ImageError;

use super::Texture;

const TEXTURE_PATH: &str = "resources/textures/";

pub struct AssetManager {
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn load_texture(&mut self, name: &str) -> Result<(), ImageError> {
        let texture = Texture::new(Path::new(&(TEXTURE_PATH.to_owned() + name)))?;
        self.textures.insert(name.to_owned(), texture);
        Ok(())
    }
}