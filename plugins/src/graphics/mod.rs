use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Msaa { samples: 8 })
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup_orthographic_camera.system())
            .run();
    }
}

fn setup_orthographic_camera(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
