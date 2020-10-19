use crate::deps::kiss3d::resource::{
    Texture,
    TextureManager,
};
use std::rc::Rc;

pub trait Resource: Sized {
    fn resource_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn raw_memory() -> &'static [u8];

    fn load() -> Rc<Texture> {
        TextureManager::get_global_manager(|tm| {
            tm.add_image_from_memory(Self::raw_memory(), Self::resource_name())
        })
    }
}


pub struct Skybox(Rc<Texture>);


impl Skybox {
    const MEMORY: &'static [u8] = include_bytes!("./assets/skybox-wasm32-bright.png");
}


impl Resource for Skybox {
    fn raw_memory() -> &'static [u8] {
        Self::MEMORY
    }
}
