export default `use bevy::{
    ecs::prelude::*,
    math::{Mat4, Quat, Vec3},
    prelude::{App, Assets, BuildChildren, Transform},
    render2::{
        camera::{OrthographicProjection, PerspectiveCameraBundle},
        color::Color,
        mesh::{shape, Mesh},
    },
    PipelinedDefaultPlugins,
};

fn main() {
    App::new()
        .insert_resource(bevy::window::WindowDescriptor {
            canvas: Some("#bevy_canvas".to_string()),
            // width: 768.0,
            // height: 432.0,
            width: 200.0,
            height: 100.0,
            ..Default::default()
        })
        .add_plugins(PipelinedDefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}`;