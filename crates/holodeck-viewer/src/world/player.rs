use crate::{
    config::Config,
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
pub struct Player {
    pub id:        Option<Id>,
    pub gfx:       GraphicsNode,
    pub color:     Color,
    pub is_client: bool,
}


impl Player {
    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn this_client(
        config: &Config,
        window: &mut Window,
    ) -> Player {
        let mut capsule = window.add_capsule(config.player_size.width(), config.player_size.height());
        capsule.append_translation(&Translation3 {
            vector: kiss3d::nalgebra::Vector3::new(
                1.0,
                config.block_size / 2.0f32 + (config.player_size.height() / 2.0f32),
                1.0,
            ),
        });

        Player {
            id:        None,
            gfx:       capsule,
            color:     config.theme.player.this_client,
            is_client: true,
        }
    }

    pub fn set_position(
        &mut self,
        pos: Point3<f32>,
    ) {
        self.scene_node_mut()
            .set_local_translation(Translation3 { vector: pos.coords });
    }
}

impl Into<Object> for Player {
    fn into(self) -> Object {
        Object::Player(self)
    }
}


impl Entity for Player {
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
