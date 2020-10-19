use crate::{
    deps::{
        kiss3d::window::Window,
        na::{
            Point3,
            Translation3,
            Vector3,
        },
    },
    engine::GraphicsNode,
    theme::Color,
    world::{
        entity::Id,
        update_graphics_node,
        Delta,
        Entity,
        Object,
    },
};

#[derive(Clone)]
pub struct Plane {
    pub id:         Option<Id>,
    pub gfx:        GraphicsNode,
    pub color:      Color,
    pub base_color: Color,
}


impl Plane {
    pub fn new_xz(
        id: Option<Id>,
        color: Color,
        xlen: f32,
        zlen: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_cube(xlen, 0.0, zlen);
        Self::with_node(node, id, color)
    }

    pub fn new_xy(
        id: Option<Id>,
        color: Color,
        xlen: f32,
        ylen: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_cube(xlen, ylen, 0.0);
        Self::with_node(node, id, color)
    }

    pub fn new_yz(
        id: Option<Id>,
        color: Color,
        ylen: f32,
        zlen: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_cube(0.0, ylen, zlen);
        Self::with_node(node, id, color)
    }

    fn with_node(
        node: GraphicsNode,
        id: Option<Id>,
        color: Color,
    ) -> Self {
        let mut plane = Plane {
            id,
            gfx: node,
            color,
            base_color: color,
        };

        plane.scene_node_mut().append_translation(&Translation3 {
            vector: Vector3::default(),
        });
        plane.set_color(color);
        // plane.scene_node_mut().append_translation(&pos);
        // plane.scene_node_mut().set_lines_width(2.0);

        plane
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


impl Into<Object> for Plane {
    fn into(self) -> Object {
        Object::Plane(self)
    }
}

impl Entity for Plane {
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
