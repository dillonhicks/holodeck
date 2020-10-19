use crate::buildmeta;

use crate::{
    geometry::AABB3,
    theme::Theme,
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Config {
    pub window_title:         Cow<'static, str>,
    pub window_size:          AABB3<f32>,
    pub block_size:           f32,
    pub player_size:          AABB3<f32>,
    pub player_move_per_step: f32,
    pub world_scale:          f32,
    pub star_size:            f32,
    pub fps_limit:            Option<u64>,
    pub world_bounds:         AABB3<f32>,
    pub theme:                Theme,
    pub field_of_view:        f32,
    pub znear:                f32,
    pub zfar:                 f32,
}


impl Config {
    pub const DEFAULT_BLOCK_SIZE: f32 = 1.0;
    pub const DEFAULT_PLAYER_MOVE_PER_STEP: f32 = 2.5;
    pub const DEFAULT_STAR_SIZE: f32 = 2.0;
    pub const DEFAULT_WORLD_SCALE: f32 = 1.0;

    pub fn default_zfar() -> f32 {
        2048.0
    }

    pub fn default_znear() -> f32 {
        0.5
    }

    pub fn default_field_of_view() -> f32 {
        800.0 / 600.0
    }

    pub fn default_player_size() -> AABB3<f32> {
        AABB3::from_origin(0.05, 0.2, 0.05)
    }

    pub fn default_world_bounds() -> AABB3<f32> {
        let radial_size: f32 = 500.0; // f32::sqrt(f32::powf(Self::default_zfar(), 2.0) / 3.0f32);
        AABB3::with_min_max(
            (-radial_size, radial_size),
            (-radial_size, radial_size),
            (-radial_size, radial_size),
        )
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            window_title:         format!(
                "{} v{} {} {}",
                buildmeta::NAME,
                buildmeta::VERSION,
                buildmeta::TARGET_OS,
                buildmeta::TARGET_ARCH
            )
            .into(),
            window_size:          AABB3::from_origin(800.0, 600.0, 0.0),
            block_size:           Self::DEFAULT_BLOCK_SIZE,
            player_size:          Config::default_player_size(),
            player_move_per_step: Self::DEFAULT_PLAYER_MOVE_PER_STEP,
            world_scale:          Self::DEFAULT_WORLD_SCALE,
            star_size:            Self::DEFAULT_STAR_SIZE,
            fps_limit:            Some(33),
            world_bounds:         Self::default_world_bounds(),
            theme:                Default::default(),
            field_of_view:        Self::default_field_of_view(),
            znear:                Self::default_znear(),
            zfar:                 Self::default_zfar(),
        }
    }
}
