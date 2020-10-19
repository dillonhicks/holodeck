use std::{
    collections::HashMap,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    thread,
    time::Duration,
};

use crate::deps::{
    holodeck_core::Result,
    holodeck_net::{
        message::{
            Entity,
            SimulationState,
            SpawnRequest,
        },
        protocol::{
            server_channel,
            BackEnd,
            Recv,
        },
        server::{
            Config,
            WebSocketServer,
        },
    },
    log::warn,
};

use crate::{
    deps::{
        log::{
            debug,
            info,
        },
        structopt::StructOpt,
        tracing::{
            info_span,
            Instrument,
        },
    },
    CommonArgs,
};

#[derive(Copy, Clone, Debug, StructOpt)]
pub struct Args {
    /// the maximum number of entities
    #[structopt(long, default_value = "250")]
    max_entities: u32,
    /// the number of entity spawns per tick, a N >= 1.0 will spawn at least
    /// int(N) entities per tick
    #[structopt(long, default_value = "0.3")]
    spawn_chance: f32,
    /// the simulation update rate
    #[structopt(long, default_value = "30.0")]
    tick_hz:      f64,
    /// the world size along each axis
    #[structopt(long, default_value = "1000.0")]
    world_size:   f32,
    /// how long to run the devserver in seconds (0 = no limit)
    #[structopt(long, default_value = "120")]
    run_seconds:  u64,
    /// the amount an entity may move per tick
    #[structopt(long, default_value = "0.5")]
    target_speed: f32,
    /// report every N ticks
    #[structopt(long, default_value = "1")]
    report_rate:  u8,
}


pub struct Server {}


impl Server {
    pub fn run(
        common: &CommonArgs,
        args: &Args,
    ) -> Result<()> {
        let server_span = info_span!("devserver");
        let _enter = server_span.enter();

        let mut config = Config::default();
        config.simulation_world_size = args.world_size;
        config.max_entities = args.max_entities as usize;
        config.tick = Duration::from_secs_f64(1.0f64 / args.tick_hz);

        info!("{:?} {:?} {:?}", common, args, config);

        let (viewer_channel, sim_channel) = server_channel();
        let run_condition = Arc::new(AtomicBool::new(true));

        let run_cond_server = run_condition.clone();
        let client_server_handle = thread::spawn(move || {
            let mut rt = crate::deps::tokio::runtime::Builder::new()
                .enable_all()
                .threaded_scheduler()
                .core_threads(2)
                .thread_name("ws-server")
                .build()
                .unwrap();

            let fut = async move {
                WebSocketServer::new(config)
                    .run_until_shutdown(viewer_channel, run_cond_server)
                    .await
                    .unwrap_or_else(|err| {
                        panic!("websocket server failed to shutdown gracefully: error={}", err)
                    });
            }
            .instrument(info_span!("server"));

            rt.block_on(fut);
        });


        // start server
        info!("simulation up and running");

        let run_cond_sim = run_condition.clone();
        let simulation_handle = {
            let args = *args;
            thread::spawn(move || {
                run_sim(&args, config, sim_channel, run_cond_sim);
            })
        };


        let run_seconds = if args.run_seconds == 0 {
            u64::MAX
        } else {
            args.run_seconds
        };

        std::thread::park_timeout(Duration::from_secs(run_seconds));

        let (tx, rx) = std::sync::mpsc::channel();

        thread::spawn(move || {
            debug!("simulation done, signaling server shutdown");
            run_condition.store(false, Ordering::SeqCst);
            info!("waiting for server to shutdown...");

            client_server_handle
                .join()
                .map_err(|err| warn!("testserver did not shutdown gracefully: {:?}", err))
                .unwrap_or(());

            simulation_handle
                .join()
                .map_err(|err| warn!("testserver did not shutdown gracefully: {:?}", err))
                .unwrap_or(());

            tx.send(()).unwrap_or_else(|err| panic!("{}", err));
        });

        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(()) => info!("shutdown gracefully, exiting"),
            Err(_) => warn!("graceful shutdown timeout, forcing shutdown"),
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

impl Velocity {
    pub fn with_random_direction(magnitude: f32) -> Self {
        let theta: f32 = 2.0f32 * std::f32::consts::PI * crate::deps::rand::random::<f32>();

        let dx = magnitude * f32::cos(theta);
        let dy = magnitude * f32::sin(theta);

        Velocity { dx, dy }
    }
}


struct Simulation {
    config:       Config,
    next_id:      u64,
    spawn_chance: f32,
    target_speed: f32,
    report_rate:  u64,
    bounds:       AABB2<f32>,
    state:        SimulationState,
    movement:     HashMap<u64, Velocity>,
    channel:      BackEnd<SimulationState, SpawnRequest>,
}


impl Simulation {
    pub fn new(
        args: &Args,
        config: Config,
        channel: BackEnd<SimulationState, SpawnRequest>,
    ) -> Self {
        let x_min = -(config.simulation_world_size / 2.0);
        let y_min = -(config.simulation_world_size / 2.0);
        let x_max = config.simulation_world_size / 2.0;
        let y_max = config.simulation_world_size / 2.0;

        Simulation {
            config,
            next_id: 1,
            spawn_chance: args.spawn_chance,
            target_speed: args.target_speed,
            report_rate: std::cmp::max(args.report_rate as u64, 1),
            bounds: AABB2::with_min_max(x_min, y_min, x_max, y_max),
            state: SimulationState {
                tick:         0,
                entity_count: 0,
                entities:     Vec::with_capacity(config.max_entities),
            },
            movement: HashMap::with_capacity(config.max_entities),
            channel,
        }
    }
}


impl Simulation {
    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn spawn_entity(
        &mut self,
        x: f32,
        y: f32,
    ) -> Option<u64> {
        if self.state.entities.len() >= self.config.max_entities {
            return None;
        }

        let id = self.next_id();
        self.state.entities.push(Entity {
            id,
            tag: 1,
            x,
            y,
            z: 0.0,
        });

        let velocity = Velocity::with_random_direction(self.target_speed);
        self.movement.insert(id, velocity);
        info!("spawned entity: {:?} {:?}", self.state.entities.last(), velocity);
        Some(id)
    }

    fn process_messages(&mut self) {
        let recv: Recv<SpawnRequest> = (&mut *self.channel).recv();
        match recv {
            Recv::Msg(msg) => {
                self.spawn_entity(msg.x, msg.y);
            }
            Recv::Empty | Recv::Disconnected | Recv::Invalid => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.state.tick += 1;

        let mut chance = self.spawn_chance;
        while chance >= 1.0 {
            chance -= 1.0;
            self.spawn_entity(0.0, 0.0);
        }

        let spawn_value: f32 = crate::deps::rand::random();
        if spawn_value < chance {
            self.spawn_entity(0.0, 0.0);
        }

        let bounds = &self.bounds;
        let entities = &mut self.state.entities[..];
        for entity in entities.iter_mut() {
            let id = entity.id;

            let velocity: &mut Velocity = self.movement.get_mut(&id).unwrap_or_else(|| panic!());

            if !bounds.contains(Vector2::new(entity.x, entity.y)) {
                let old_vel = *velocity;

                if entity.x < bounds.min.x {
                    entity.x = bounds.min.x;
                    velocity.dx = -velocity.dx;
                } else if entity.x > bounds.max.x {
                    entity.x = bounds.max.x;
                    velocity.dx = -velocity.dx;
                }
                if entity.y < bounds.min.y {
                    entity.y = bounds.min.y;
                    velocity.dy = -velocity.dy;
                } else if entity.y > bounds.max.y {
                    entity.y = bounds.max.y;
                    velocity.dy = -velocity.dy;
                }
                debug!("bounced entity: {:?}; {:?} -> {:?}", entity, old_vel, velocity);
            }

            entity.x += velocity.dx;
            entity.y += velocity.dy;
        }

        let state = &self.state;
        let channel = &mut self.channel;
        if state.tick % self.report_rate == 0 {
            channel.send(&state);
        }
    }
}


fn run_sim(
    args: &Args,
    config: Config,
    sim_channel: BackEnd<SimulationState, SpawnRequest>,
    running: Arc<AtomicBool>,
) {
    let mut simulation = Simulation::new(args, config, sim_channel);


    'update: while running.load(Ordering::Relaxed) {
        let start = std::time::Instant::now();
        let end = start + config.tick;

        simulation.on_tick();

        while std::time::Instant::now() < end {
            std::sync::atomic::spin_loop_hint();
        }
    }
}


#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


impl<T> Vector2<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn new(
        x: T,
        y: T,
    ) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}


impl Vector2<f32> {
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

impl<T> std::convert::From<[T; 2]> for Vector2<T>
where
    T: Copy,
{
    fn from(arr: [T; 2]) -> Vector2<T> {
        let [x, y] = arr;
        Vector2::new(x, y)
    }
}

impl std::ops::Add for Vector2<f32> {
    type Output = Vector2<f32>;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vector2<f32> {
    type Output = Vector2<f32>;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct AABB2<T> {
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}

impl<T> AABB2<T>
where
    T: Copy + PartialOrd,
{
    pub fn with_min_max(
        min_x: T,
        min_y: T,
        max_x: T,
        max_y: T,
    ) -> Self {
        Self {
            min: Vector2::new(min_x, min_y),
            max: Vector2::new(max_x, max_y),
        }
    }

    #[inline]
    pub fn contains(
        &self,
        point: Vector2<T>,
    ) -> bool {
        self.min.x <= point.x && point.x < self.max.x && self.min.y <= point.y && point.y < self.max.y
    }

    pub fn intersects(
        &self,
        other: &Self,
    ) -> bool {
        (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
    }
}

impl AABB2<f32> {
    pub const MAX: Self = AABB2 {
        min: Vector2 {
            x: f32::MIN,
            y: f32::MIN,
        },
        max: Vector2 {
            x: f32::MAX,
            y: f32::MAX,
        },
    };

    #[inline]
    pub fn center(&self) -> Vector2<f32> {
        let rel_mid_x = (self.max.x - self.min.x) / 2.0f32;
        let mid_x = rel_mid_x + self.min.x;

        let rel_mid_y = (self.max.y - self.min.y) / 2.0f32;
        let mid_y = rel_mid_y + self.min.y;

        Vector2::new(mid_x, mid_y)
    }

    #[inline]
    pub fn y_extent(&self) -> Vector2<f32> {
        Vector2::new(self.min.y, self.max.y)
    }

    #[inline]
    pub fn x_extent(&self) -> Vector2<f32> {
        Vector2::new(self.min.x, self.max.x)
    }
}
