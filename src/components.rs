use specs::prelude::*;
use specs_derive::Component;

use vector2d::Vector2D;

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
}

// a renderable item and details about the image
#[derive(Component)]
pub struct Renderable {
    // the name of the texture to be rendered
    pub tex_name: String,
    // width of the src
    pub i_w: u32,
    // height of the src
    pub i_h: u32,
    // width of the dest
    pub o_w: u32,
    // height of the dest
    pub o_h: u32,
    // offset number of widths to crop
    pub frame: u32,
    // max frame offset before
    pub total_frames: u32,
    // rotation of the image to display
    pub rot: f64,
}

#[derive(Component)]
pub struct Player {
    pub impulse: Vector2D<f64>,
    pub current_speed: Vector2D<f64>,
}