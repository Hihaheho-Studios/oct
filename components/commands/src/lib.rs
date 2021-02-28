use bevy_components::prelude::{KeyCode, Mut, Transform, Vec3};

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

pub fn handle_action(player: &mut RigidBody, camera: &Transform, action: &Action) {
    use Action::*;
    let pos = camera.translation;
    let forward = camera.forward();
    let mut looking = forward - pos;
    looking.y = 0.0;
    let movement = looking.normalize();
    let movement = match *action {
        MoveForward => movement,
        MoveLeft => Vec3::new(movement.z, movement.y, -movement.x),
        MoveRight => Vec3::new(-movement.z, movement.y, movement.x),
        MoveBack => -movement,
    };
    println!("{:?}", movement);
    player.apply_force(Vector::new(movement.x, movement.y, movement.z), true);
}
