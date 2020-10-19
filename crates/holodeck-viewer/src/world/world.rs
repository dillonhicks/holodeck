use std::collections::HashMap;

use crate::{
    config::Config,
    deps::{
        holodeck_core::messages::SimulationState,
        kiss3d::{
            light::Light,
            nalgebra::{
                Point3,
                Translation3,
                Vector3,
            },
            resource::MeshManager,
            scene::SceneNode,
            window::Window,
        },
        na::{
            Unit,
            UnitQuaternion,
        },
    },
    engine::GraphicsNode,
    geometry::AABB3,
    texture,
    texture::Resource,
    theme::Color,
    world::{
        Block,
        Border2,
        DynamicEntity,
        Gridlines,
        Id,
        Object,
        Quad,
        Shell,
        Zones,
    },
};

pub type Tick = u64;
pub type Tag = u16;


pub struct Environment {
    // pub skybox:  Skybox,
    pub skyshell: Shell,
    pub grid:     Gridlines,
    pub ground:   Quad,
    pub boundary: Border2,
    pub zones:    Vec<Quad>,
}

impl Environment {
    fn draw(
        &mut self,
        window: &mut Window,
    ) {
        self.grid.draw(window);
        self.boundary.draw(window);
        let rot = UnitQuaternion::from_euler_angles(0.00004, -0.00003, 0.00006);
        self.skyshell.scene_node_mut().append_rotation(&rot);
    }

    fn update(
        &mut self,
        mut point: Point3<f32>,
    ) {
        // the skybox and skyshell should always be relative to the camera
        self.skyshell.set_position(point);
        // but the ground should stay stationary
        point.y = 0.0;
        self.grid.set_position(point);
        self.ground.set_position(point);
    }

    fn toggle_zones(&mut self) {
        let toggle = !self.zones[0].scene_node().is_visible();
        self.zones
            .iter_mut()
            .for_each(|z| z.scene_node_mut().set_visible(toggle))
    }
}


pub struct World {
    // pub player: Player,
    pub updated:     bool,
    pub tick:        Tick,
    pub bounds:      AABB3<f32>,
    pub position:    Point3<f32>,
    pub direction:   Vector3<f32>,
    pub environment: Environment,
    pub objects:     HashMap<Id, DynamicEntity>,
    pub tag_colors:  HashMap<Tag, Color>,
}


impl World {
    pub fn generate(
        config: &Config,
        window: &mut Window,
    ) -> Self {
        window.set_light(Light::StickToCamera);


        let grid = Gridlines {
            color:    Color::holodeck_grid(),
            delta:    Vector3::new(0.0f32, 0.0f32, 0.0f32),
            step:     Vector3::new(100.0f32, 100.0f32, 100.0f32),
            bounds:   config.world_bounds,
            zfar:     config.zfar,
            position: Vector3::new(0.0f32, 0.11f32, 0.0f32),
        };

        let mut skyshell = Shell::new(
            config.zfar * 0.9f32,
            Color::white(),
            Point3::new(0.0f32, 0.0, 0.0),
            window,
        );

        skyshell.scene_node_mut().set_texture(texture::Skybox::load());


        let x_len = config.world_bounds.width() / 2.0f32;
        let z_len = config.world_bounds.depth() / 2.0f32;

        let mut boundary = Border2::new(
            None,
            Vector3::new(0.0f32, 2.0f32, 0.0f32),
            Vector3::x_axis().scale(-x_len),
            Vector3::z_axis().scale(z_len),
            Color::holodeck_boundary(),
        );
        boundary.set_layers(5);
        boundary.set_spacing(5.0f32);
        let mut zones = Zones::generate(&config.world_bounds, window);

        let mut tag_colors = HashMap::new();

        for (i, color) in Color::holodeck_zones().iter().copied().enumerate() {
            let id = i as u16;
            tag_colors.insert(id, color);
        }

        let mut world = Self {
            // player,
            updated: false,
            tick: 0,
            bounds: config.world_bounds,
            //  skybox,
            position: Point3::new(0.0f32, 0.0f32, 0.0f32),
            direction: Vector3::default(),
            environment: Environment {
                skyshell,
                grid,
                ground: Quad::new_xz(
                    None,
                    config.theme.global.plane,
                    Point3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
                    config.zfar * 2.0,
                    config.zfar * 2.0,
                    window,
                ),
                boundary,
                zones,
            },
            objects: Default::default(),
            tag_colors,
        };
        world.toggle_zones();

        world
    }

    pub fn on_tick(&mut self) {
        self.updated = false;
    }

    pub fn process(
        &mut self,
        state: Box<SimulationState>,
        window: &mut Window,
    ) {
        let tick = state.tick;

        let tag_colors = &self.tag_colors;
        for crate::deps::holodeck_core::messages::Entity { id, x, y, z, tag } in
            state.entities.iter().copied()
        {
            let pos = Point3::new(x - 500.0, 1.2f32, y - 500.0).into();

            let entry = self.objects.entry(id).or_insert_with(|| {
                let color = tag_colors.get(&tag).copied().unwrap_or(Color::white());
                let object = Object::Block(Block::new_cube(Some(id), pos, color.into(), 2.25f32, window));

                DynamicEntity::new(object, tick, tag)
            });


            entry.update(tick, pos);
            if tag != entry.tag {
                let color = tag_colors.get(&tag).copied().unwrap_or(Color::white());
                entry.set_color(color);
            }
            entry.tick = tick;
        }

        self.tick = tick;
        self.updated = true;
    }

    pub fn draw(
        &mut self,
        window: &mut Window,
    ) {
        self.environment.draw(window);

        if self.updated == false {
            let tick = self.tick;
            #[cfg(feature = "interpolation")]
            for entity in self.objects.values_mut().filter(|e| e.tick < tick) {
                entity.interpolate();
            }
        }
    }

    pub fn toggle_zones(&mut self) {
        self.environment.toggle_zones();
    }

    pub fn translate(
        &mut self,
        _point: Point3<f32>,
    ) {
    }

    pub fn update_position_and_direction(
        &mut self,
        mut point: Point3<f32>,
        dir: Vector3<f32>,
    ) {
        // self.skybox.set_position(point);
        self.direction = dir;
        self.position = point;
        self.environment.update(self.position);
    }
}
