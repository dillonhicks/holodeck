use crate::deps::{
    kiss3d::nalgebra as na,
    na::{
        Point3,
        RealField,
        Scalar,
    },
};
use std::fmt;

#[inline(always)]
pub fn spherical_to_cartesian(
    r: f32,
    theta: f32,
    phi: f32,
) -> Point3<f32> {
    let x = r * f32::sin(theta) * f32::cos(phi);
    let y = r * f32::sin(theta) * f32::sin(phi);
    let z = r * f32::cos(theta);

    Point3::new(x, y, z)
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB3<S: Scalar + fmt::Debug> {
    pub min: na::Point3<S>,
    pub max: na::Point3<S>,
}



impl<S> AABB3<S>
where
    S: Scalar + RealField + fmt::Debug + Default,
{
    pub fn from_origin(
        width: S,
        height: S,
        depth: S,
    ) -> Self {
        AABB3 {
            min: na::Point3::new(S::default(), S::default(), S::default()),
            max: na::Point3::new(width, height, depth),
        }
    }

    pub fn with_min_max(
        xs: (S, S),
        ys: (S, S),
        zs: (S, S),
    ) -> Self {
        AABB3 {
            min: na::Point3::new(xs.0, ys.0, zs.0),
            max: na::Point3::new(xs.1, ys.1, zs.1),
        }
    }

    pub fn width(&self) -> S {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> S {
        self.max.y - self.min.y
    }

    pub fn depth(&self) -> S {
        self.max.z - self.min.z
    }
}


impl AABB3<f32> {
    pub fn magnitude(&self) -> f32 {
        let x = self.width();
        let y = self.height();
        let z = self.width();

        f32::sqrt(f32::powf(x, 2.0) + f32::powf(y, 2.0) + f32::powf(z, 2.0))
    }

    pub fn radius(&self) -> f32 {
        let x = self.width() / 2.0;
        let y = self.height() / 2.0;
        let z = self.width() / 2.0;

        f32::sqrt(f32::powf(x, 2.0) + f32::powf(y, 2.0) + f32::powf(z, 2.0))
    }
}
