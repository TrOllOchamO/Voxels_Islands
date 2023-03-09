use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;

use game::camera::FirstPersonCamera;
use game::world::world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FirstPersonCamera { has_focus: true })
        .add_plugin(game::world::world::Dimention)
        .add_startup_system(setup_system)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

// set lighting
fn setup_system(mut commands: Commands) {
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 0.10,
    // });

    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 32_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}
