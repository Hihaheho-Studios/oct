use std::{intrinsics::transmute, mem::transmute_copy};

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::camera::{Camera, CameraProjection, PerspectiveProjection},
};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app //.add_resource(Msaa { samples: 1 })
            .add_plugin(EguiPlugin)
            .add_startup_system(setup_camera.system())
            .add_system(camera_movement.system())
            .add_system(ui_example.system())
            .run();
    }
}

fn ui_example(
    mut projection: Query<(&mut Camera, &mut PerspectiveProjection, &mut Transform)>,
    mut egui_context: ResMut<EguiContext>,
) {
    let (mut camera, mut projection, mut transform) = projection.iter_mut().next().unwrap();
    let ctx = &mut egui_context.ctx;
    egui::Window::new("Hello").show(ctx, |ui| {
        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.add(egui::Slider::f32(&mut projection.far, 0.0..=10.0).text("far"));
            ui.add(egui::Slider::f32(&mut projection.near, 0.0..=10.0).text("near"));
            ui.add(egui::Slider::f32(&mut projection.aspect_ratio, 0.1..=3.0).text("aspect_ratio"));
            ui.add(egui::Slider::f32(&mut projection.fov, 0.0..=2.0).text("fov"));

            ui.add(egui::Slider::f32(&mut transform.translation.x, 0.0..=10.0).text("x"));
            ui.add(egui::Slider::f32(&mut transform.translation.y, 0.0..=10.0).text("y"));
            ui.add(egui::Slider::f32(&mut transform.translation.z, 0.0..=10.0).text("z"));
        });
    });

    camera.projection_matrix = projection.get_projection_matrix();
}

fn setup_camera(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            perspective_projection: PerspectiveProjection {
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(5.0, 5.0, -5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

fn camera_movement(mut query: Query<&mut Transform, With<Camera>>) {
    let mut transform = query.iter_mut().next().unwrap();
    transform.translation = Mat4::from_rotation_y(0.015).transform_vector3(transform.translation);
    transform.look_at(Vec3::default(), Vec3::unit_y());
}
