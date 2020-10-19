use crate::{
    deps::{
        kiss3d::{
            resource::Texture,
            window::Window,
        },
        na::{
            Point3,
            Translation3,
            Vector3,
        },
    },
    geometry::{
        spherical_to_cartesian,
        AABB3,
    },
    theme::Color,
    world::Quad,
};
use std::rc::Rc;

pub struct Skybox {
    moon_dist: f32,
    // moon:      Sphere,
    planes:    [Quad; 6],
    xlen:      f32,
    ylen:      f32,
    zlen:      f32,
}

fn calculate_skybox_transforms(
    origin: Point3<f32>,
    xlen: f32,
    ylen: f32,
    zlen: f32,
) -> [Translation3<f32>; 6] {
    [
        Translation3 {
            vector: Vector3::new(origin.x, origin.y + (ylen / 2.0), origin.z),
        },
        Translation3 {
            vector: Vector3::new(origin.x, origin.y + (-ylen / 2.0), origin.z),
        },
        Translation3 {
            vector: Vector3::new(origin.x, origin.y, origin.z + (zlen / 2.0)),
        },
        Translation3 {
            vector: Vector3::new(origin.x, origin.y, origin.z + (-zlen / 2.0)),
        },
        Translation3 {
            vector: Vector3::new(origin.x + (xlen / 2.0), origin.y, origin.z),
        },
        Translation3 {
            vector: Vector3::new(origin.x + (-xlen / 2.0), origin.y, origin.z),
        },
    ]
}

impl Skybox {
    fn moon_position(moon_dist: f32) -> Point3<f32> {
        spherical_to_cartesian(moon_dist, std::f32::consts::PI / 4.0, std::f32::consts::PI / 4.0)
    }

    pub fn from_bounds(
        bounds: &AABB3<f32>,
        window: &mut Window,
    ) -> Self {
        let moon_dist = 100.0;

        // let mut moon = Sphere::new(
        //     30.0,
        //     Color::holodeck_sky_light(),
        //     Self::moon_position(moon_dist),
        //     window,
        // );
        // // disable the special material effects for the moon when running in the browser
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     let material = Rc::new(RefCell::new(
        //         Box::new(NormalMaterial::new()) as Box<dyn Material + 'static>
        //     ));
        //     moon.scene_node_mut().set_material(material);
        // }

        let xlen = bounds.width();
        let ylen = bounds.height();
        let zlen = bounds.depth();

        let origin = Point3::new(0.0f32, 0.0, 0.0);

        let mut this = Self {
            moon_dist,
            // moon,
            planes: [
                Quad::new_xz(None, Color::white(), origin, xlen, zlen, window),
                Quad::new_xz(None, Color::white(), origin, xlen, zlen, window),
                Quad::new_xy(None, Color::white(), origin, xlen, ylen, window),
                Quad::new_xy(None, Color::white(), origin, xlen, ylen, window),
                Quad::new_yz(None, Color::white(), origin, ylen, zlen, window),
                Quad::new_yz(None, Color::white(), origin, ylen, zlen, window),
            ],
            xlen,
            ylen,
            zlen,
        };

        this.set_position(Point3::<f32>::new(0.0, 0.0, 0.0));
        this
    }

    pub fn set_texture(
        &mut self,
        texture: Rc<Texture>,
    ) {
        for plane in self.planes.iter_mut() {
            plane.scene_node_mut().set_texture(texture.clone());
        }
    }

    pub fn set_position(
        &mut self,
        pos: Point3<f32>,
    ) {
        // let moon_position = Self::moon_position(self.moon_dist).coords + pos.coords;
        // self.moon.scene_node_mut().set_local_translation(Translation3 {
        //     vector: moon_position,
        // });

        let transforms = calculate_skybox_transforms(pos, self.xlen, self.ylen, self.zlen);
        self.planes
            .iter_mut()
            .enumerate()
            .for_each(|(i, plane)| plane.scene_node_mut().set_local_translation(transforms[i]));
    }
}


// impl IntoIterator for Skybox {
//     type Item = Plane;
//     type IntoIter = <SmallVec<[Plane; 6]> as IntoIterator>::IntoIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.planes.into_iter()
//     }
// }
