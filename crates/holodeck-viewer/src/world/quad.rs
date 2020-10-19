use crate::{
    deps::{
        kiss3d::{
            resource::Texture,
            window::Window,
        },
        na::{
            Point3,
            Translation3,
            UnitQuaternion,
            Vector3,
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
use std::rc::Rc;

#[derive(Clone)]
pub struct Quad {
    pub id:         Option<Id>,
    pub gfx:        GraphicsNode,
    pub color:      Color,
    pub base_color: Color,
}


impl Quad {
    pub fn new_xy(
        id: Option<Id>,
        color: Color,
        pos: Point3<f32>,
        w: f32,
        h: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_quad(w, h, 1, 1);
        let mut quad = Quad {
            id,
            gfx: node,
            color,
            base_color: color,
        };
        quad.set_color(color);
        quad.scene_node_mut()
            .append_translation(&Translation3 { vector: pos.coords });
        quad.scene_node_mut().enable_backface_culling(true);
        // block.scene_node_mut().set_lines_width(2.0);

        quad
    }

    pub fn new_yz(
        id: Option<Id>,
        color: Color,
        pos: Point3<f32>,
        ylen: f32,
        zlen: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_quad(ylen, zlen, 1, 1);
        let mut quad = Quad {
            id,
            gfx: node,
            color,
            base_color: color,
        };
        quad.set_color(color);
        let theta = Vector3::new(0.0f32, 0.5 * std::f32::consts::PI, 0.0);
        quad.scene_node_mut()
            .append_rotation_wrt_center(&UnitQuaternion::new(theta));
        quad.scene_node_mut()
            .append_translation(&Translation3 { vector: pos.coords });
        // block.scene_node_mut().set_lines_width(2.0);
        quad.scene_node_mut().enable_backface_culling(true);

        quad
    }

    pub fn new_xz(
        id: Option<Id>,
        color: Color,
        pos: Point3<f32>,
        xlen: f32,
        ylen: f32,
        window: &mut Window,
    ) -> Self {
        let node = window.add_quad(xlen, ylen, 1, 1);
        let mut quad = Quad {
            id,
            gfx: node,
            color,
            base_color: color,
        };
        quad.set_color(color);
        let theta = Vector3::new(-0.5f32 * std::f32::consts::PI, 0.0, 0.0);
        quad.scene_node_mut()
            .append_rotation_wrt_center(&UnitQuaternion::new(theta));
        quad.scene_node_mut()
            .append_translation(&Translation3 { vector: pos.coords });
        // block.scene_node_mut().set_lines_width(2.0);
        quad.scene_node_mut().enable_backface_culling(true);

        quad
    }

    pub fn square(
        id: Option<Id>,
        pos: Point3<f32>,
        color: Color,
        side: f32,
        window: &mut Window,
    ) -> Self {
        Self::new_xz(id, color, pos, side, side, window)
    }

    fn from_node(
        node: GraphicsNode,
        color: Option<Color>,
    ) -> Quad {
        let color = color.unwrap_or_else(Color::white);

        Quad {
            id: None,
            gfx: node,
            color,
            base_color: color,
        }
    }

    // pub fn from_bounds(
    //     bounds: &AABB3<f32>,
    //     window: &mut Window,
    // ) -> Quad {
    //     let xlen = bounds.width();
    //     //let ylen = bounds.height();
    //     let zlen = bounds.depth();
    //     let mut node = window.add_quad(xlen, zlen);
    //     Quad::from_node(node, None)
    // }

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

impl Into<Object> for Quad {
    fn into(self) -> Object {
        Object::Quad(self)
    }
}


impl Entity for Quad {
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
