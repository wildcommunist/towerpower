mod tower;
mod game_assets;
mod target;
mod bullet;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::bullet::BulletPlugin;
use crate::game_assets::GameAssets;
use crate::target::{Health, Target, TargetPlugin};
use crate::tower::{Tower, TowerPlugin};

pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const GAME_VERSION: &str = "v0.0.1";

fn main() {
    App::new()
        // Yes! The order of plugins and resources matters
        .insert_resource(ClearColor(Color::rgb(0.39, 0.58, 0.93))) // Cornflower blue, XNA nostalgia
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                resizable: false,
                title: format!("TowerPower - BEVY Tower Defence Game ({})", GAME_VERSION),
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)

        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)

        .run();
}

fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        bullet: assets.load("models/bullet.glb#Scene0"),
        mob_spawn_delay: Timer::from_seconds(2., TimerMode::Repeating),
    });
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    ).insert(Name::new("Camera"));
}

//pbr bundle - Physically base rendering
/*
IMPORTANT! If you request a mutable resources, it that system cannot run in parallel, so only request
it when needed.
 */
fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.52).into()),
        ..default()
    }).insert(Name::new("Plane"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.9, 0.54, 0.52).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Cube"));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 750.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 8.0, 4.0),
        ..default()
    }).insert(Name::new("Light"));
}