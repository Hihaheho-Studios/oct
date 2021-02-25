use bevy::prelude::*;
use plugins::{DiagnosticsPlugin, GraphicsPlugin};

fn main() {
    App::build()
        .add_plugin(GraphicsPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
