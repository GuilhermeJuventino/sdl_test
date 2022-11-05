use specs::{World, WorldExt, Builder, Join};
use std::collections::HashMap;
use vector2d::Vector2D;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::utils::rad_to_deg;

const ROTATION_SPEED: f64 = 1.5;
const MAX_SPEED: f64 = 3.5;
const ACCELERATION: f64 = 4.5;
const DECELERATION: f64 = 0.99;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut positions = ecs.write_storage::<crate::components::Position>();
    let mut players = ecs.write_storage::<crate::components::Player>();

    for (player, pos) in (&mut players, &mut positions).join() {
        if crate::utils::is_key_pressed(&key_manager, "Left") {
            pos.rot -= ROTATION_SPEED;
        }
        if crate::utils::is_key_pressed(&key_manager, "Right") {
            pos.rot += ROTATION_SPEED;
        }

        update_movement(pos, player);
        if crate::utils::is_key_pressed(&key_manager, "Up") {
            let radians = pos.rot.to_radians();
            let move_x = radians.sin() * ACCELERATION;
            let move_y = radians.cos() * ACCELERATION;

            let move_vec = Vector2D::<f64>::new(move_x, move_y);

            player.impulse += move_vec;
        }

        if pos.rot > 360.0 {
            pos.rot -= 360.0;
        }
        if pos.rot < 0.0 {
            pos.rot += 360.0;
        }
    }
}

pub fn update_movement(
    pos: &mut crate::components::Position,
    player: &mut crate::components::Player,
) {
    player.current_speed *= DECELERATION;

    player.current_speed += player.impulse;
    if player.current_speed.length() > MAX_SPEED {
        player.current_speed = player.current_speed.normalise();
        player.current_speed = player.current_speed * MAX_SPEED;
    }

    pos.x += player.current_speed.x;
    pos.y -= player.current_speed.y;

    player.impulse = Vector2D::new(0.0, 0.0);
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(crate::components::Position{ x: SCREEN_WIDTH as f64 / 2.0, y: SCREEN_HEIGHT as f64 / 2.0, rot: 0.0 })
        .with(crate::components::Renderable{
            tex_name: String::from("img/space_ship.png"),
            i_w: 100,
            i_h: 100,
            o_w: 50,
            o_h: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(crate::components::Player{
            impulse: Vector2D::new(0.0, 0.0),
            current_speed: Vector2D::new(0.0, 0.0),
        })
        .build();
}