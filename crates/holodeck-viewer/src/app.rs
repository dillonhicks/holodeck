#[cfg(feature = "ui")]
use crate::ui::{
    Draw,
    DrawEvent,
};
use crate::{
    client::{
        Client,
        RunMode,
    },
    config::Config,
    deps::{
        holodeck_core::messages::{
            SimulationState,
            SpawnRequest,
        },
        kiss3d::{
            camera::Camera,
            event::{
                Action,
                Event,
                Key,
                Modifiers,
                MouseButton,
                WindowEvent,
            },
            light::Light,
            nalgebra::Vector3,
            planar_camera::PlanarCamera,
            post_processing::PostProcessingEffect,
            window::{
                CanvasSetup,
                NumSamples,
                State,
                Window,
            },
        },
        log::{
            debug,
            info,
        },
        na,
        na::{
            Point2,
            Translation3,
        },
    },
    engine,
    engine::{
        CameraEffects,
        GraphicsManager,
    },
    theme::Color,
    world,
};

pub type ViewerChannel = Box<dyn BackendChannel<Tx = SpawnRequest, Rx = Box<SimulationState>>>;

const ICON: &'static [u8] = include_bytes!("./holodeck.png");


pub trait BackendChannel {
    type Tx;
    type Rx;

    fn send(
        &self,
        value: Self::Tx,
    );


    fn recv(&self) -> Option<Self::Rx>;
}


pub struct PlayerGameClient {
    config:     Config,
    world:      world::World,
    client:     Client,
    graphics:   engine::GraphicsManager,
    cursor_pos: Point2<f32>,
    frontend:   Option<ViewerChannel>,
    #[cfg(feature = "ui")]
    hud:        crate::ui::HeadsUpDisplay,
}


impl PlayerGameClient {
    pub fn run(frontend: Option<ViewerChannel>) {
        let config = Config::default();

        let canvas_setup = CanvasSetup {
            vsync: true,
            #[cfg(target_arch = "wasm32")]
            samples: NumSamples::Zero,
            #[cfg(not(target_arch = "wasm32"))]
            samples: NumSamples::Two,
        };

        let mut window = Window::new_with_setup(
            config.window_title.as_ref(),
            config.window_size.width() as u32,
            config.window_size.height() as u32,
            canvas_setup,
        );

        let icon = crate::deps::image::load_from_memory(ICON).unwrap();

        window.set_icon(icon);
        window.set_background_color(1.0, 1.0, 1.0);
        let Color(r, g, b, ..) = Color::black();
        window.set_background_color(r, g, b);
        window.set_framerate_limit(config.fps_limit);
        window.set_light(Light::StickToCamera);

        let world = world::World::generate(&config, &mut window);
        let graphics = GraphicsManager::with_config(&config);
        #[cfg(feature = "ui")]
        let ui = crate::ui::HeadsUpDisplay::new(&mut window);

        window.render_loop(PlayerGameClient {
            config,
            world,
            client: Default::default(),
            graphics,
            cursor_pos: Point2::new(0.0f32, 0.0),
            frontend,
            #[cfg(feature = "ui")]
            hud: ui,
        });
    }

    fn handle_common_event<'b>(
        &mut self,
        event: Event<'b>,
        window: &mut Window,
    ) -> Event<'b> {
        debug!("event: {:?}", event.value);

        match event.value {
            WindowEvent::Key(Key::T, Action::Release, _) => {
                if self.client.run_mode == RunMode::Paused {
                    self.client.run_mode = RunMode::Running;
                } else {
                    self.client.run_mode = RunMode::Paused;
                }
            }
            WindowEvent::Key(Key::S, Action::Release, _) => self.client.run_mode = RunMode::Running,
            // WindowEvent::Key(Key::Escape, Action::Release, _) => self.client.run_mode = RunMode::Paused,
            WindowEvent::Key(Key::P, Action::Release, _) => {
                self.hud.minimap_mut().toggle_hidden();
            }
            WindowEvent::Key(Key::Add, Action::Release, _) => {
                self.hud.minimap_mut().toggle_size();
            }
            WindowEvent::Key(Key::R, Action::Release, _) => {
                // self
                //     .state
                //     .action_flags
                //     .set(TestbedActionFlags::EXAMPLE_CHANGED, true)
            }
            WindowEvent::Key(Key::C, Action::Release, _) => {
                // // Delete 1 collider of 10% of the remaining dynamic bodies.
                // let mut colliders: Vec<_> = self
                //     .physics
                //     .bodies
                //     .iter()
                //     .filter(|e| e.1.is_dynamic())
                //     .filter(|e| !e.1.colliders().is_empty())
                //     .map(|e| e.1.colliders().to_vec())
                //     .collect();
                // colliders.sort_by_key(|co| -(co.len() as isize));

                // let num_to_delete = (colliders.len() / 10).max(1);
                // for to_delete in &colliders[..num_to_delete] {
                //     self.physics
                //         .colliders
                //         .remove(to_delete[0], &mut self.physics.bodies);
                // }
            }

            WindowEvent::Key(Key::D, Action::Release, _) => {
                info!("entities={}", self.world.objects.len());
            }
            WindowEvent::Key(Key::PageUp, Action::Release, _) => {
                let step = self.graphics.camera().move_step();
                self.graphics.camera_mut().set_move_step(step * 2.0f32);
            }
            WindowEvent::Key(Key::PageDown, Action::Release, _) => {
                let step = self.graphics.camera().move_step();
                self.graphics.camera_mut().set_move_step(step / 2.0f32);
            }
            WindowEvent::Key(Key::O, Action::Release, _) => {
                self.world.toggle_zones();
            }
            WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _modifier) => {
                let size = window.size();
                let (pos, dir) = self
                    .graphics
                    .camera()
                    .unproject(&self.cursor_pos, &na::convert(size));
                let camera_pos = self.graphics.camera().eye();

                info!(
                    "cursor_pos={:?}; world_pos={:?}, ray_start={:?}, ray_dir={:?}",
                    self.cursor_pos, camera_pos, pos, dir
                );
            }
            WindowEvent::CursorPos(x, y, _) => {
                self.cursor_pos.x = x as f32;
                self.cursor_pos.y = y as f32;
            }

            WindowEvent::Key(Key::Space, Action::Release, modifiers)
                if modifiers.contains(Modifiers::Control) =>
            {
                let eye = self.graphics.camera().eye();

                self.graphics
                    .camera_mut()
                    .set_move_step(self.config.player_move_per_step);

                self.graphics.camera_mut().translate_mut(&Translation3 {
                    vector: Vector3::new(-eye.x, 0.2f32 - eye.y, -eye.z),
                });
            }
            WindowEvent::Key(Key::Left, Action::Press, _) => {
                if self.client.run_mode == RunMode::Running {
                    let (up, down, left, right) = (false, false, true, false);
                    let direction = self.graphics.camera().move_dir(up, down, left, right);
                    let magnitude = self.config.player_move_per_step;
                    let movement = direction * magnitude;
                    self.world.translate(movement.into());
                }
            }
            WindowEvent::Key(Key::Right, Action::Press, _) => {
                if self.client.run_mode == RunMode::Running {
                    let (up, down, left, right) = (false, false, false, true);
                    let direction = self.graphics.camera().move_dir(up, down, left, right);
                    let magnitude = self.config.player_move_per_step;
                    let movement = direction * magnitude;
                    self.world.translate(movement.into());
                }
            }
            WindowEvent::Key(Key::Up, Action::Press, _) => {
                if self.client.run_mode == RunMode::Running {
                    let (up, down, left, right) = (true, false, false, false);
                    let direction = self.graphics.camera().move_dir(up, down, left, right);
                    let magnitude = self.config.player_move_per_step;
                    let movement = direction * -magnitude;
                    self.world.translate(movement.into());
                }
            }
            WindowEvent::Key(Key::Down, Action::Press, _) => {
                if self.client.run_mode == RunMode::Running {
                    let (up, down, left, right) = (false, true, false, false);
                    let direction = self.graphics.camera().move_dir(up, down, left, right);
                    let magnitude = self.config.player_move_per_step;
                    let movement = direction * -magnitude;
                    self.world.translate(movement.into());
                }
            }
            _ => {}
        }

        event
    }

    fn do_move(
        &mut self,
        _direction: Vector3<f32>,
    ) {
    }
}


impl State for PlayerGameClient {
    fn cameras_and_effect_and_renderer(&mut self) -> CameraEffects<'_> {
        self.graphics.camera_and_effects_and_renderer()
    }

    fn step(
        &mut self,
        window: &mut Window,
    ) {
        for event in window.events().iter() {
            self.handle_common_event(event, window);
        }
        let direction = self.graphics.camera().eye_dir();
        let point = self.graphics.camera().eye();


        let mut last_state = None;
        while let Some(msg) = self.frontend.as_mut().and_then(|fe| fe.recv()) {
            last_state = Some(msg);
        }

        self.world.on_tick();
        if let Some(state) = last_state {
            self.world.process(state, window);
        } else {
            self.world.tick += 1;
        }
        self.world.update_position_and_direction(point, direction);
        self.world.draw(window);


        self.graphics.on_update();

        #[cfg(feature = "ui")]
        {
            let eye_dir = self.graphics.camera().eye_dir();
            let eye = self.graphics.camera().eye();

            let hud = &mut self.hud;
            let w = window.width();
            let h = window.height();
            let world = &self.world;
            let mut ui_cell = window.conrod_ui_mut().set_widgets();
            let ui = &mut ui_cell;


            let mut event = DrawEvent {
                eye: &eye,
                eye_dir: &eye_dir,
                world,
                w,
                h,
                ui,
            };


            hud.draw(&mut event);
        }
    }
}
