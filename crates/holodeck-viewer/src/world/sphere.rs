use crate::{
    deps::{
        kiss3d::window::Window,
        na::{
            Point3,
            Translation3,
        },
    },
    engine::GraphicsNode,
    theme::Color,
    world::{
        update_graphics_node,
        Delta,
        Entity,
        Id,
        Object,
    },
};

#[derive(Clone)]
pub struct Sphere {
    pub id:    Option<Id>,
    pub gfx:   GraphicsNode,
    pub color: Color,
}


impl Sphere {
    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn new(
        radius: f32,
        color: Color,
        pos: Point3<f32>,
        window: &mut Window,
    ) -> Sphere {
        let mut sphere = window.add_sphere(radius);

        sphere.append_translation(&Translation3 { vector: pos.coords });
        let mut sphere = Sphere {
            id: None,
            gfx: sphere,
            color,
        };

        sphere.set_color(color);
        sphere
    }

    pub fn set_color(
        &mut self,
        color: Color,
    ) {
        let Color(r, g, b, ..) = color;
        self.gfx.set_color(r, g, b);
        self.color = color;
    }
}

impl Into<Object> for Sphere {
    fn into(self) -> Object {
        Object::Sphere(self)
    }
}

impl Entity for Sphere {
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
