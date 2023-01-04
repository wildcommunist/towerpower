mod tower;
mod game_assets;
mod target;
mod bullet;
mod physics;
mod camera;
mod ui;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use crate::bullet::BulletPlugin;
use crate::camera::CameraPlugin;
use crate::game_assets::GameAssets;
use crate::physics::PhysicsPlugin;
use crate::target::{Target, TargetPlugin};
use crate::tower::{TowerPlugin};
use crate::ui::GameUiPlugin;

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

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPickingPlugins)

        .add_plugin(CameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GameUiPlugin)

        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_basic_scene)

        .run();
}

fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        bullet: assets.load("models/bullet.glb#Scene0"),
        pedestal: assets.load("models/pedestal.glb#Scene0"),
        tower: assets.load("models/tower_1.glb#Scene0"),
        enemy: assets.load("models/enemy.glb#Scene0"),
        mob_spawn_delay: Timer::from_seconds(1.5, TimerMode::Repeating),

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
    )
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
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
    assets: Res<AssetServer>,
) {
    let default_collider_color: Handle<StandardMaterial> = materials.add(
        Color::rgba(0.3, 0.3, 0.3, 0.3).into()
    );
    let selected_collider_color: Handle<StandardMaterial> = materials.add(
        Color::rgba(0.3, 0.9, 0.3, 0.9).into()
    );

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.52).into()),
        ..default()
    }).insert(Name::new("Plane"));

    // Create an empty, to store our children
    commands.spawn(SpatialBundle::from_transform(
        Transform::from_xyz(0.0, 0.8, 0.0)
    ))
        .insert(Name::new("Tower_base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color),
        })
        .insert(default_collider_color)
        .with_children(|commands| {
            // Tower pedestal
            commands.spawn(SceneBundle {
                scene: assets.load("models/pedestal.glb#Scene0"),
                transform: Transform::from_xyz(0.0, -0.9, 0.0),
                ..default()
            })
                .insert(Name::new("Pedestal"));
        });

    // Light
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