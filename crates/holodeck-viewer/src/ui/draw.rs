use crate::deps::kiss3d::conrod;

use crate::{
    deps::na::{
        Point3,
        Vector3,
    },
    world::World,
};

pub trait Draw {
    fn draw(
        &mut self,
        event: &mut DrawEvent<'_, '_>,
    );
}


pub struct DrawEvent<'a, 'b> {
    pub eye:     &'a Point3<f32>,
    pub eye_dir: &'a Vector3<f32>,
    pub world:   &'a World,
    pub w:       u32,
    pub h:       u32,

    // pub window:  &'a mut Window,
    pub ui: &'a mut conrod::UiCell<'b>,
}
