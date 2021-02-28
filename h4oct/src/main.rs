use bevy::prelude::*;
use plugins::{DiagnosticsPlugin, GraphicsPlugin, InputsPlugin, PhysicsPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(InputsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GraphicsPlugin)
        .run();
}
