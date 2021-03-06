use bevy_components::prelude::{KeyCode, Vec3};

use glam::Mat4;
use rapier3d_components::{dynamics::RigidBody, math::Vector};
use strum::{EnumString, IntoStaticStr};

pub struct Player;

#[derive(EnumString, IntoStaticStr)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBack,
}

pub enum Mapping {
    // Mouse {
    //     axis: MouseAxis,
    //     dead_zone: f32,
    //     mapping: Input,
    // },
    Key { keycode: KeyCode, input: Action },
}

pub fn default_mappings() -> Vec<Mapping> {
    use Action::*;
    use Mapping::*;
    vec![
        Key {
            keycode: KeyCode::A,
            input: MoveLeft,
        },
        Key {
            keycode: KeyCode::S,
            input: MoveRight,
        },
        Key {
            keycode: KeyCode::W,
            input: MoveForward,
        },
        Key {
            keycode: KeyCode::R,
            input: MoveBack,
        },
    ]
}

pub fn handle_action(player: &mut RigidBody, transform: Mat4, action: &Action) {
    use Action::*;
    let mut movement = transform
        .inverse()
        .transform_vector3(Vec3::new(0.0, 0.0, 1.0));
    movement.y = 0.0;
    let movement = movement.normalize() * 2.0;
    let movement = match *action {
        MoveForward => movement,
        MoveLeft => Vec3::new(movement.z, movement.y, -movement.x),
        MoveRight => Vec3::new(-movement.z, movement.y, movement.x),
        MoveBack => -movement,
    };
    println!("{:?}", movement);
    player.apply_impulse(Vector::new(movement.x, movement.y, movement.z), true);
}
