use bevy::prelude::*;
use bevy_mod_picking::{PickingCameraBundle, Selection};

const CAMERA_SPEED: f32 = 3.0;
const CAMERA_ROTATE_SPEED: f32 = 1.65;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system(camera_controls)
            .add_system(what_is_selected)
        ;
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    )
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
}


fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    let mut left = camera.left();
    forward.y = 0.0;
    left.y = 0.0;
    forward = forward.normalize();
    left = left.normalize();

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * CAMERA_SPEED;
    }

    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * CAMERA_SPEED;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * CAMERA_SPEED;
    }

    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * CAMERA_SPEED;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, CAMERA_ROTATE_SPEED * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -CAMERA_ROTATE_SPEED * time.delta_seconds());
    }
}

fn what_is_selected(
    selection: Query<(&Name, &Selection)>
) {
    for (name, selection) in &selection {
        if selection.selected() {
            // this is what we have selected
        }
    }
}