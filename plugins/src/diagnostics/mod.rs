use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(PrintDiagnosticsPlugin::default())
            .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default());
    }
}
