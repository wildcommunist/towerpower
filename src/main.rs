mod tower;
mod game_assets;
mod target;
mod bullet;
mod physics;
mod camera;
mod ui;
mod states;
mod menu;
mod player;
mod pause;
mod helpers;
mod gameplay;

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy_mod_picking::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use crate::bullet::BulletPlugin;
use crate::camera::CameraPlugin;
use crate::game_assets::GameAssets;
use crate::gameplay::GameplayPlugin;
use crate::menu::MainMenuPlugin;
use crate::pause::PauseGamePlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::states::GameState;
use crate::target::{TargetPlugin};
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
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin::new())

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)

        .add_state(GameState::Gameplay)

        .add_plugin(MainMenuPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PauseGamePlugin)
        .add_plugin(GameplayPlugin)

        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)

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
        game_font: assets.load("fonts/minecraft_font.ttf"),
        enemy_death_sounds: assets.load("sounds/pop-39222.ogg"),
        tower_place_sound: assets.load("sounds/bricks-104933.ogg"),
    });
}