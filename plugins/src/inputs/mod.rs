use std::str::FromStr;

use bevy::prelude::*;
use bevy::{prelude::Plugin, render::camera::Camera};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};
use commands::{default_mappings, handle_action, Action, Mapping, Player};
use kurinji::{Kurinji, KurinjiPlugin, MouseAxis, OnActionActive, OnActionBegin};

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .add_startup_system(initialize_mapping.system())
            .add_system(input_system.system());
    }
}

fn initialize_mapping(mut kurinji: ResMut<Kurinji>) {
    for mapping in default_mappings() {
        match mapping {
            Mapping::Key { keycode, input } => kurinji.bind_keyboard_pressed(keycode, input.into()),
        };
    }
}

fn input_system(
    commands: &mut Commands,
    camera: Query<(&Transform, &Camera)>,
    mut player: Query<(&mut RigidBodyHandleComponent, &Player)>,
    mut bodies: ResMut<RigidBodySet>,
    mut active_reader: Local<EventReader<OnActionActive>>,
    active: Res<Events<OnActionActive>>,
) {
    let (camera_transform, camera) = camera.iter().next().unwrap();
    let (player_handle, _player) = player.iter_mut().next().unwrap();
    for event in active_reader.iter(&active) {
        let input: Action = FromStr::from_str(event.action.as_str()).unwrap();
        let body = bodies.get_mut(player_handle.handle()).unwrap();
        handle_action(
            body,
            camera.projection_matrix * camera_transform.compute_matrix().inverse(),
            &input,
        );
    }
}
