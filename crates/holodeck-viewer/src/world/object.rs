use crate::{
    engine::GraphicsNode,
    world::{
        Block,
        Delta,
        Entity,
        Plane,
        Player,
        Quad,
        Sphere,
    },
};

pub enum Object {
    Block(Block),
    Player(Player),
    Plane(Plane),
    Sphere(Sphere),
    Quad(Quad),
    Entity(Box<dyn Entity>),
}

impl Object {
    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        match self {
            Object::Block(entity) => entity.scene_node_mut(),
            Object::Player(entity) => entity.scene_node_mut(),
            Object::Plane(entity) => entity.scene_node_mut(),
            Object::Sphere(entity) => entity.scene_node_mut(),
            Object::Quad(entity) => entity.scene_node_mut(),
            Object::Entity(_entity) => panic!(),
        }
    }
}

impl Entity for Object {
    fn scene_node(&self) -> &GraphicsNode {
        match self {
            Object::Block(entity) => entity.scene_node(),
            Object::Player(entity) => entity.scene_node(),
            Object::Plane(entity) => entity.scene_node(),
            Object::Sphere(entity) => entity.scene_node(),
            Object::Quad(entity) => entity.scene_node(),
            Object::Entity(entity) => entity.scene_node(),
        }
    }

    fn apply_delta(
        &mut self,
        delta: &Delta,
    ) {
        match self {
            Object::Block(entity) => entity.apply_delta(delta),
            Object::Player(entity) => entity.apply_delta(delta),
            Object::Plane(entity) => entity.apply_delta(delta),
            Object::Sphere(entity) => entity.apply_delta(delta),
            Object::Quad(entity) => entity.apply_delta(delta),
            Object::Entity(entity) => entity.apply_delta(delta),
        }
    }
}
