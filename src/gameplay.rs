use anyhow::{anyhow, Context};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_picking::{Highlighting, PickableBundle};
use crate::helpers::*;
use crate::player::Player;
use crate::states::GameState;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    //.with_system(load_assets)
                    .with_system(spawn_basic_scene)
            )
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        ;
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameMap {
    pub starting_lives: u32,
    pub starting_funds: u32,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub waypoints: Vec<Vec2>,
}

#[derive(Component)]
pub struct GroundPlane;

fn load_assets(
    mut commands: Commands
) {
    info!("Loading the map...");
    let map = match GameMap::load("./assets/levels/test.ldtk") {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to load game map! {}", e);
            return;
        }
    };

    //TODO: Show map load error dialog and push to main menu

    commands.insert_resource(map);
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
    map: Res<GameMap>,
) {
    let default_collider_color: Handle<StandardMaterial> = materials.add(
        Color::rgba(0.3, 0.3, 0.3, 0.3).into()
    );
    let selected_collider_color: Handle<StandardMaterial> = materials.add(
        Color::rgba(0.3, 0.9, 0.3, 0.9).into()
    );

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(GameFieldGround { width: map.width, height: map.height })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.52).into()),
        ..default()
    }).insert((Name::new("Ground"), GroundPlane));

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
            selected: Some(selected_collider_color.clone()),
        })
        .insert(default_collider_color.clone())
        .with_children(|commands| {
            // Tower pedestal
            commands.spawn(SceneBundle {
                scene: assets.load("models/pedestal.glb#Scene0"),
                transform: Transform::from_xyz(0.0, -0.9, 0.0),
                ..default()
            })
                .insert(Name::new("Pedestal"));
        });

    commands.spawn(SpatialBundle::from_transform(
        Transform::from_xyz(1.5, 0.8, 0.0)
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