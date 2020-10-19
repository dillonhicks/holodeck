use crate::{
    deps::{
        kiss3d::window::Window,
        na::{
            Point3,
            Translation3,
        },
    },
    engine::GraphicsNode,
    geometry::AABB3,
    theme::Color,
    world::{
        grid::static_grid_range,
        update_graphics_node,
        Delta,
        Entity,
        Id,
        Quad,
    },
};

pub struct Zones;

impl Zones {
    pub fn generate(
        bounds: &AABB3<f32>,
        window: &mut Window,
    ) -> Vec<Quad> {
        let step_x = 200.0f32;
        let step_z = 200.0f32;


        let xs = static_grid_range(0.0f32, bounds.width() - step_x, step_x);
        let zs = static_grid_range(0.0f32, bounds.depth() - step_z, step_z);

        let mut colors = Color::holodeck_zones().into_iter().cycle();

        let _half_x = step_x / 2.0;
        let _half_z = step_z / 2.0;

        let mut zones = vec![];
        for x in xs {
            for z in zs {
                let mut quad = Quad::new_xz(
                    None,
                    colors.next().unwrap(),
                    Point3::new(x + step_x, 0.1, z + step_z),
                    step_x,
                    step_z,
                    window,
                );
                // quad.scene_node_mut().set_visible(false);
                zones.push(quad);
            }
        }

        zones
    }
}
