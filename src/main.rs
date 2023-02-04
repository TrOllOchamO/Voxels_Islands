use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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

// set the light
fn setup_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.08,
    });
}
