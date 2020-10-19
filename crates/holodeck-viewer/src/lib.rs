#![allow(dead_code)]
#![cfg_attr(debug_assertions, allow(unused))]
pub(crate) mod deps {
    #[cfg(feature = "interpolation")]
    pub(crate) use arraydeque;
    pub(crate) use holodeck_core;
    pub(crate) use image;
    pub(crate) use kiss3d::{
        self,
        nalgebra as na,
    };
    pub(crate) use log;
}


pub mod app;
mod buildmeta;
mod client;
pub mod config;
mod engine;
mod geometry;
#[cfg(not(target_arch = "wasm32"))]
mod material;
mod texture;
mod theme;
#[cfg(feature = "ui")]
mod ui;
mod world;
