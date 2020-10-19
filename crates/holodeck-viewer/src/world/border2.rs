use crate::{
    deps::{
        kiss3d::window::Window,
        na,
        na::{
            Point3,
            Translation3,
            Unit,
            Vector3,
        },
    },
    engine::GraphicsNode,
    theme::Color,
    world::entity::Id,
};
use std::num::NonZeroU16;

pub struct Border2 {
    id:      Option<Id>,
    normal:  Unit<Vector3<f32>>,
    points:  [Point3<f32>; 4],
    w:       Vector3<f32>,
    color:   Point3<f32>,
    layers:  Option<NonZeroU16>,
    spacing: f32,
    gfx:     GraphicsNode,
}


impl Border2 {
    pub fn new(
        id: Option<Id>,
        r0: Vector3<f32>,
        v: Vector3<f32>,
        w: Vector3<f32>,
        color: Color,
    ) -> Self {
        let gfx = GraphicsNode::new_empty();
        // let g = v + w;
        let normal = Unit::new_normalize(v.cross(&w));
        let w = v + w;
        use std::f32::consts::{
            FRAC_PI_2,
            PI,
        };

        let points = [
            (r0 + w).into(),
            (r0 + na::Rotation::from_axis_angle(&normal, FRAC_PI_2) * w).into(),
            (r0 + na::Rotation::from_axis_angle(&normal, PI) * w).into(),
            (r0 + na::Rotation::from_axis_angle(&normal, 3.0f32 * FRAC_PI_2) * w).into(),
        ];
        Border2 {
            id,
            normal,
            points,
            w,
            color: color.into(),
            layers: None,
            spacing: 1.0,
            gfx,
        }
    }

    pub fn set_position(
        &mut self,
        pos: Point3<f32>,
    ) {
        self.scene_node_mut()
            .set_local_translation(Translation3 { vector: pos.coords });
    }

    pub fn set_layers(
        &mut self,
        n: u16,
    ) {
        self.layers = NonZeroU16::new(n)
    }

    pub fn set_spacing(
        &mut self,
        spacing: f32,
    ) {
        self.spacing = spacing.abs();
    }

    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn draw(
        &self,
        window: &mut Window,
    ) {
        let points = &self.points;
        let normal = self.normal;
        let color = &self.color;
        let spacing = self.spacing;

        window.draw_line(&points[0], &points[1], color);
        window.draw_line(&points[1], &points[2], color);
        window.draw_line(&points[2], &points[3], color);
        window.draw_line(&points[3], &points[0], color);

        let layers = self.layers.map(|n| n.get()).unwrap_or_default();
        for n in 1..layers {
            let n = n as f32;
            let translation = normal.scale(n * spacing);
            let points: [Point3<f32>; 4] = [
                (translation + points[0].coords).into(),
                (translation + points[1].coords).into(),
                (translation + points[2].coords).into(),
                (translation + points[3].coords).into(),
            ];
            window.draw_line(&points[0], &points[1], color);
            window.draw_line(&points[1], &points[2], color);
            window.draw_line(&points[2], &points[3], color);
            window.draw_line(&points[3], &points[0], color);
        }
    }
}
