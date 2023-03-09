use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::PI;

const MOBILITY_SPEED: f32 = 20.;
const CAMERA_SENSIBILITY: f32 = 0.005;

#[derive(Component, Default)]
pub struct FirstPersonCamera {
    pub has_focus: bool,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

fn spawn_first_person_camera_system(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle { ..default() })
        .insert(FirstPersonCamera { has_focus: true })
        .insert(Velocity {
            ..Default::default()
        })
        .insert(Rotation {
            yaw: PI / 2.,
            pitch: -PI / 8.,
            roll: 0.,
        })
        .insert(Position {
            x: 3.0,
            y: 1.5,
            z: 0.0,
        });
}

fn update_camera_velocity_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&FirstPersonCamera, &mut Velocity, &Rotation)>,
) {
    for (camera, mut velocity, camera_rotation) in query.iter_mut() {
        if camera.has_focus {
            *velocity = Velocity {
                x: 0.,
                y: 0.,
                z: 0.,
            };

            if keys.pressed(KeyCode::D) {
                velocity.x += MOBILITY_SPEED * (camera_rotation.yaw + PI / 2.).sin();
                velocity.z += MOBILITY_SPEED * (camera_rotation.yaw + PI / 2.).cos();
            }
            if keys.pressed(KeyCode::Q) {
                velocity.x -= MOBILITY_SPEED * (camera_rotation.yaw + PI / 2.).sin();
                velocity.z -= MOBILITY_SPEED * (camera_rotation.yaw + PI / 2.).cos();
            }
            if keys.pressed(KeyCode::Space) {
                velocity.y += MOBILITY_SPEED;
            }
            if keys.pressed(KeyCode::LShift) {
                velocity.y -= MOBILITY_SPEED;
            }
            if keys.pressed(KeyCode::Z) {
                velocity.x -= MOBILITY_SPEED * camera_rotation.yaw.sin();
                velocity.z -= MOBILITY_SPEED * camera_rotation.yaw.cos();
            }
            if keys.pressed(KeyCode::S) {
                velocity.x += MOBILITY_SPEED * camera_rotation.yaw.sin();
                velocity.z += MOBILITY_SPEED * camera_rotation.yaw.cos();
            }
        }
    }
}

fn update_camera_position_system(
    time: Res<Time>,
    mut query: Query<(&FirstPersonCamera, &Velocity, &mut Position)>,
) {
    for (camera, velocity, mut position) in query.iter_mut() {
        if camera.has_focus {
            position.x += velocity.x * time.delta_seconds();
            position.y += velocity.y * time.delta_seconds();
            position.z += velocity.z * time.delta_seconds();
        }
    }
}

fn move_camera_system(mut query: Query<(&Position, &mut Transform), With<FirstPersonCamera>>) {
    for (position, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x = position.x;
        translation.y = position.y;
        translation.z = position.z;
    }
}

fn update_camera_rotation_system(
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&FirstPersonCamera, &mut Rotation)>,
) {
    for ev in motion_evr.iter() {
        for (camera, mut camera_rotation) in query.iter_mut() {
            if camera.has_focus {
                camera_rotation.yaw -= ev.delta.x * CAMERA_SENSIBILITY;
                camera_rotation.pitch -= ev.delta.y * CAMERA_SENSIBILITY;

                if camera_rotation.yaw > 2. * PI {
                    camera_rotation.yaw -= 2. * PI;
                }
                if camera_rotation.yaw < -2. * PI {
                    camera_rotation.yaw += 2. * PI;
                }
                if camera_rotation.pitch > PI / 2. {
                    camera_rotation.pitch = PI / 2.;
                }
                if camera_rotation.pitch < -PI / 2. {
                    camera_rotation.pitch = -PI / 2.;
                }
            }
        }
    }
}

fn rotate_camera_system(mut query: Query<(&Rotation, &mut Transform), With<FirstPersonCamera>>) {
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.yaw, rotation.pitch, 0.);
    }
}

fn cursor_grab_system(
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut query: Query<&mut FirstPersonCamera>,
) {
    let Ok(mut primary) = primary_query.get_single_mut() else {
        return;
    };
    for mut camera in query.iter_mut() {
        if btn.just_pressed(MouseButton::Right) {
            primary.cursor.grab_mode = CursorGrabMode::Locked;
            primary.cursor.visible = false;
            camera.has_focus = true;
        }

        if key.just_pressed(KeyCode::Escape) {
            primary.cursor.grab_mode = CursorGrabMode::None;
            primary.cursor.visible = true;
            camera.has_focus = false;
        }
    }
}

impl Plugin for FirstPersonCamera {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_first_person_camera_system)
            .add_system(cursor_grab_system)
            .add_system(update_camera_velocity_system)
            .add_system(update_camera_position_system)
            .add_system(move_camera_system)
            .add_system(update_camera_rotation_system)
            .add_system(rotate_camera_system);
    }
}
