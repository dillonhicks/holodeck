use crate::{
    deps::{
        kiss3d::{
            resource::Texture,
            window::Window,
        },
        na::{
            Point3,
            Translation3,
        },
    },
    engine::GraphicsNode,
    geometry::AABB3,
    theme::Color,
    world::{
        entity::Id,
        update_graphics_node,
        Delta,
        Entity,
        Object,
    },
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Block {
    pub id:         Option<Id>,
    pub gfx:        GraphicsNode,
    pub color:      Color,
    pub base_color: Color,
}


impl Block {
    pub fn new_cube(
        id: Option<Id>,
        pos: Point3<f32>,
        color: Color,
        size: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_cube(size, size, size);
        let mut block = Block {
            id,
            gfx: node,
            color,
            base_color: color,
        };
        block.set_color(color);
        block
            .scene_node_mut()
            .append_translation(&Translation3 { vector: pos.coords });
        // block.scene_node_mut().set_lines_width(2.0);

        block
    }

    fn from_node(
        node: GraphicsNode,
        color: Option<Color>,
    ) -> Block {
        let color = color.unwrap_or_else(Color::white);

        Block {
            id: None,
            gfx: node,
            color,
            base_color: color,
        }
    }

    pub fn from_bounds(
        bounds: &AABB3<f32>,
        window: &mut Window,
    ) -> Block {
        let xlen = bounds.width();
        let ylen = bounds.height();
        let zlen = bounds.depth();
        let node = window.add_cube(xlen, ylen, zlen);
        let color: Option<Color> = None;
        Block::from_node(node, color)
    }

    pub fn set_texture(
        &mut self,
        texture: Rc<Texture>,
    ) {
        self.scene_node_mut().set_texture(texture)
    }

    pub fn select(&mut self) {
        // todo: config?
        self.color = Color::red();
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }

    pub fn set_color(
        &mut self,
        color: Color,
    ) {
        let Color(r, g, b, ..) = color;
        self.gfx.set_color(r, g, b);
        self.color = color;
        self.base_color = color;
    }

    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn set_position(
        &mut self,
        point: Point3<f32>,
    ) {
        self.scene_node_mut()
            .set_local_translation(Translation3 { vector: point.coords });
    }
}

impl Into<Object> for Block {
    fn into(self) -> Object {
        Object::Block(self)
    }
}

impl Entity for Block {
    fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    fn apply_delta(
        &mut self,
        delta: &Delta,
    ) {
        let id = &self.id;
        let gfx = &mut self.gfx;
        let color = &self.color;

        id.as_ref().map(|id| update_graphics_node(id, gfx, delta, color));
    }
}
