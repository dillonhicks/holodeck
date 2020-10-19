use crate::deps::kiss3d::{
    conrod,
    nalgebra::Point3,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn from_int_rgba(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Self {
        Self(
            r as f32 / Self::MAX_RGBA,
            g as f32 / Self::MAX_RGBA,
            b as f32 / Self::MAX_RGBA,
            a as f32 / Self::MAX_RGBA,
        )
    }
}


// colors
impl Color {
    const MAX_RGBA: f32 = 255.0f32;

    pub fn with_r(
        mut self,
        r: f32,
    ) -> Self {
        self.0 = r;
        self
    }

    pub fn with_g(
        mut self,
        g: f32,
    ) -> Self {
        self.1 = g;
        self
    }

    pub fn with_b(
        mut self,
        b: f32,
    ) -> Self {
        self.2 = b;
        self
    }

    pub fn with_a(
        mut self,
        a: f32,
    ) -> Self {
        self.3 = a;
        self
    }

    pub const fn black() -> Color {
        Color(0.0, 0.0, 0.0, 1.0)
    }

    pub const fn white() -> Color {
        Color(1.0, 1.0, 1.0, 1.0)
    }

    pub const fn red() -> Color {
        Color(1.0, 0.0, 0.0, 1.0)
    }

    pub const fn green() -> Color {
        Color(0.0, 1.0, 0.0, 1.0)
    }

    pub const fn blue() -> Color {
        Color(0.0, 0.0, 1.0, 1.0)
    }

    pub fn mustard() -> Color {
        Color::from_int_rgba(197, 160, 0, 255)
    }

    pub const fn sky_blue() -> Color {
        Color(0.529, 0.808, 0.98, 1.0)
    }

    pub const fn borg_grey() -> Color {
        Color(0.80, 0.80, 0.85, 1.0)
    }

    pub fn holodeck_sky() -> Color {
        Color::from_int_rgba(51, 8, 63, 255)
    }

    pub fn holodeck_sky_light() -> Color {
        Color::from_int_rgba(255, 50, 255, 255)
    }

    pub fn holodeck_ground() -> Color {
        Color::from_int_rgba(12, 75, 120, 255)
    }

    pub fn holodeck_grid() -> Color {
        Color::from_int_rgba(78, 183, 255, 255)
    }

    pub fn holodeck_energy() -> Color {
        Color::from_int_rgba(255, 231, 32, 255)
    }

    pub fn holodeck_plasma() -> Color {
        Color::from_int_rgba(255, 0, 200, 255)
    }

    pub fn holodeck_space_grey() -> Color {
        Color::from_int_rgba(33, 47, 60, 255)
    }

    pub fn holodeck_zones() -> Vec<Color> {
        vec![
            [235, 103, 150],
            [80, 188, 235],
            [235, 233, 56],
            [89, 240, 101],
            [192, 162, 235],
            [229, 169, 235],
            [145, 229, 235],
            //[235, 232, 122],
            [179, 235, 134],
            // cyan
            [65, 248, 250],
            // purple
            [140, 165, 250],
            // magenta/hot-pink
            [250, 41, 126],
            // orange
            [250, 103, 15],
            // white
            [255, 255, 255],
            // grey
            [180, 180, 180],
            // red
            [255, 0, 0],
            // green
            [0, 255, 0],
            // blue
            [0, 0, 255],
            [159, 91, 235],
        ]
        .into_iter()
        .map(|c| c.into())
        .collect::<Vec<_>>()
    }

    pub fn holodeck_boundary() -> Color {
        Color::from_int_rgba(186, 83, 230, 255)
    }
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        let [r, g, b] = rgb;
        Color::from_int_rgba(r, g, b, 255)
    }
}


impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        let [r, g, b, a] = rgba;
        Color::from_int_rgba(r, g, b, a)
    }
}

impl AsRef<Point3<f32>> for Color {
    fn as_ref(&self) -> &Point3<f32> {
        let ptr = self as *const Color as *const u8;
        unsafe { &*(ptr as *const Point3<f32>) }
    }
}

impl Into<Point3<f32>> for Color {
    fn into(self) -> Point3<f32> {
        Point3::from([self.0, self.1, self.2])
    }
}

impl Into<conrod::Color> for Color {
    fn into(self) -> conrod::Color {
        conrod::Color::Rgba(self.0, self.1, self.2, self.3)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Global {
    pub skybox: Color,
    pub plane:  Color,
}


#[derive(Copy, Clone, Debug)]
pub struct Player {
    pub this_client: Color,
}


#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub global: Global,
    pub player: Player,
}


impl Default for Theme {
    fn default() -> Self {
        Theme {
            global: Global {
                skybox: Color::holodeck_sky(),
                plane:  Color::holodeck_ground(),
            },
            player: Player {
                this_client: Color::borg_grey(),
            },
        }
    }
}
