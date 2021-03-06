use bevy::prelude::*;
use bevy_rapier3d::{
    physics::{EventQueue, RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Vector},
    render::RapierRenderPlugin,
};
use commands::Player;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_plugin(RapierRenderPlugin)
            .add_resource(RapierConfiguration {
                gravity: Vector::y() * -9.8,
                scale: 1.0,
                physics_pipeline_active: true,
                query_pipeline_active: true,
                ..Default::default()
            })
            .add_system(collision.system())
            .add_startup_system(spawn.system())
            .add_resource(EventQueue::new(false));
    }
}

fn spawn(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            RigidBodyBuilder::new_dynamic().translation(2.0, 2.5, 0.0),
            ColliderBuilder::ball(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ))
        .spawn((
            RigidBodyBuilder::new_dynamic().translation(-3.0, 0.5, 0.0),
            ColliderBuilder::cuboid(1.0, 1.0, 1.0),
            Transform::default(),
            GlobalTransform::default(),
            Player,
        ))
        .spawn((
            RigidBodyBuilder::new_static(),
            ColliderBuilder::cuboid(10.0, 0.1, 10.0),
            Transform::default(),
            GlobalTransform::default(),
        ));
}

fn collision(events: Res<EventQueue>) {
    while let Ok(event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}
