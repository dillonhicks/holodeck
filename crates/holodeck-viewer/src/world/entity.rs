#[cfg(feature = "interpolation")]
use crate::deps::arraydeque::{
    behavior,
    ArrayDeque,
};
use crate::{
    deps::na::{
        Point3,
        Translation3,
        UnitQuaternion,
        Vector3,
    },
    engine::GraphicsNode,
    theme::Color,
    world::{
        Block,
        Object,
        Plane,
        Player,
        Quad,
        Sphere,
    },
};

pub type Id = u64;


pub trait Entity {
    fn scene_node(&self) -> &GraphicsNode;

    fn apply_delta(
        &mut self,
        delta: &Delta,
    );
}


#[cfg(feature = "interpolation")]
type Positions = ArrayDeque<[(u64, Vector3<f32>); VelocityInterpolation::SAMPLES], behavior::Wrapping>;

#[cfg(feature = "interpolation")]
#[derive(Default)]
pub(crate) struct VelocityInterpolation {
    pub repeated_calls: u32,
    pub velocity:       Option<Vector3<f32>>,
    pub positions:      Positions,
}


#[cfg(feature = "interpolation")]
impl VelocityInterpolation {
    const MIN_SAMPLES: usize = 2;
    const SAMPLES: usize = 5;

    pub fn insert(
        &mut self,
        tick: u64,
        pos: Vector3<f32>,
    ) {
        self.positions.push_back((tick, pos));
        self.velocity = None;
        self.repeated_calls = 0;
    }

    pub fn velocity(&mut self) -> Vector3<f32> {
        self.repeated_calls += 1;

        let samples = self.positions.len();
        if samples < Self::MIN_SAMPLES {
            return Vector3::default();
        }

        let mut start_tick = 0;
        let mut last_tick = 0;
        let mut last_pos = Vector3::default();

        let mut total_velocity = Vector3::<f32>::default();
        let mut iter = self.positions.iter().peekable();

        if let Some((tick, pos)) = iter.next() {
            start_tick = *tick;
            last_tick = *tick;
            last_pos = *pos;
        }

        for (tick, pos) in iter {
            let tick_delta = tick - last_tick;
            total_velocity += (pos - last_pos) / (tick_delta as f32);

            last_tick = *tick;
            last_pos = *pos;
        }

        total_velocity / ((samples - 1) as f32)
    }
}


pub struct DynamicEntity {
    pub(super) object:        Object,
    pub(super) tick:          u64,
    pub(super) tag:           u16,
    #[cfg(feature = "interpolation")]
    pub(super) interpolation: VelocityInterpolation,
}

impl DynamicEntity {
    pub fn new(
        object: Object,
        tick: u64,
        tag: u16,
    ) -> Self {
        DynamicEntity {
            object,
            tick,
            tag,
            #[cfg(feature = "interpolation")]
            interpolation: Default::default(),
        }
    }
}


impl DynamicEntity {
    #[cfg(feature = "interpolation")]
    pub fn interpolate(&mut self) {
        let translation = self.object.scene_node().data().local_translation();

        self.object.scene_node_mut().set_local_translation(Translation3 {
            vector: translation.vector + self.interpolation.velocity(),
        });

        // let Color(r, g, b, _a) = Color::borg_grey();
        // self.object.scene_node_mut().set_color(r, g, b);
    }

    pub fn set_color(
        &mut self,
        color: Color,
    ) {
        let Color(r, g, b, a) = color;
        self.object.scene_node_mut().set_color(r, g, b);
    }

    pub fn update(
        &mut self,
        tick: u64,
        position: Point3<f32>,
    ) {
        #[cfg(feature = "interpolation")]
        {
            self.interpolation.insert(tick, position.coords);
        }

        self.apply_delta(&Delta::Position { id: 0, position });
    }
}

impl Entity for DynamicEntity {
    fn scene_node(&self) -> &GraphicsNode {
        self.object.scene_node()
    }

    fn apply_delta(
        &mut self,
        delta: &Delta,
    ) {
        match delta {
            Delta::Position { id: _, position } => {
                // self.velocity = position.coords -
                // self.object.scene_node().data().local_translation().vector;
                self.object.apply_delta(delta);
            }
            _ => {}
        }
    }
}

impl Into<Object> for DynamicEntity {
    fn into(self) -> Object {
        Object::Entity(Box::new(self))
    }
}


#[derive(Debug)]
pub enum Delta {
    Position {
        id:       Id,
        position: Point3<f32>,
    },
    Rotation {
        id:       Id,
        rotation: UnitQuaternion<f32>,
    },
    Create {
        id:   Id,
        kind: Kind,
    },
    Delete {
        id: Id,
    },
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Block = 1,
    Player = 2,
    Plane = 3,
    Sphere = 4,
    Quad = 5,
}


impl Delta {
    pub fn position(
        id: Id,
        position: Point3<f32>,
    ) -> Self {
        Self::Position { id, position }
    }

    pub fn rotation(
        id: Id,
        rotation: UnitQuaternion<f32>,
    ) -> Self {
        Self::Rotation { id, rotation }
    }

    pub fn create(
        id: Id,
        kind: Kind,
    ) -> Self {
        Self::Create { id, kind }
    }

    pub fn delete(id: Id) -> Self {
        Self::Delete { id }
    }
}



pub fn update_graphics_node(
    id: &Id,
    node: &mut GraphicsNode,
    delta: &Delta,
    color: &Color,
) {
    match delta {
        Delta::Position { position, .. } => {
            node.set_local_translation(Translation3 {
                vector: position.coords,
            });
        }
        Delta::Rotation { rotation, .. } => node.set_local_rotation(*rotation),
        Delta::Delete { id: to_delete } => {
            assert_eq!(*id, *to_delete);
            node.unlink()
        }
        Delta::Create { .. } => unreachable!(),
    }
    let Color(r, g, b, ..) = *color;
    node.set_color(r, g, b);
}
