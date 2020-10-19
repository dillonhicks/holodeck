use crate::{
    deps::{
        kiss3d::window::Window,
        na::{
            Point3,
            Vector3,
        },
    },
    geometry::AABB3,
    theme::Color,
};

#[derive(Debug)]
pub struct Gridlines {
    pub color:    Color,
    pub delta:    Vector3<f32>,
    pub step:     Vector3<f32>,
    pub bounds:   AABB3<f32>,
    pub zfar:     f32,
    pub position: Vector3<f32>,
}


impl Gridlines {
    fn xs(&self) -> RangeF32 {
        static_grid_range(self.position.x, self.zfar * 2.0, self.step.x)
    }

    fn zs(&self) -> RangeF32 {
        static_grid_range(self.position.z, self.zfar * 2.0, self.step.z)
    }

    pub fn draw(
        &self,
        window: &mut Window,
    ) {
        let pos_x = self.position.x;
        let pos_z = self.position.z;

        for x in self.xs() {
            let a = Point3::new(x, self.position.y, -self.zfar + pos_z);
            let b = Point3::new(x, self.position.y, self.zfar + pos_z);
            window.draw_line(&a, &b, self.color.as_ref());
        }

        for z in self.zs() {
            let a = Point3::new(-self.zfar + pos_x, self.position.y, z);
            let b = Point3::new(self.zfar + pos_x, self.position.y, z);
            window.draw_line(&b, &a, self.color.as_ref());
        }

        // window.draw_line(&a, &b, self.color.as_ref());
    }

    pub fn set_position(
        &mut self,
        pos: Point3<f32>,
    ) {
        self.position.x = pos.coords.x;
        self.position.z = pos.coords.z;
    }
}


#[derive(Copy, Clone, Debug)]
pub struct RangeF32 {
    pub start: f32,
    pub end:   f32,
    pub step:  f32,
}

impl Iterator for RangeF32 {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let value = self.start;
            self.start += self.step;
            Some(value)
        } else {
            None
        }
    }
}

pub fn static_grid_range(
    single_axis_position: f32,
    axis_len: f32,
    step: f32,
) -> RangeF32 {
    let start = single_axis_position - (axis_len / 2.0);
    let delta = start.rem_euclid(step);
    let start = start - delta;
    let end = start + axis_len;

    let (start, end) = if end > start { (start, end) } else { (end, start) };

    RangeF32 { start, end, step }
}
