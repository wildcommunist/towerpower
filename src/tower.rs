use bevy::prelude::*;
use bevy::time::Timer;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_mod_picking::Selection;
use crate::bullet::{Bullet, Lifetime};
use crate::game_assets::GameAssets;
use crate::physics::PhysicsBundle;
use crate::states::GameState;
use crate::target::{Target};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
    pub range: f32,
}

#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Lazer,
    Cannon,
    Rock,
}

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Lazer => (
                assets.tower.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                    bullet_offset: Vec3::ZERO,
                    range: 4.5,
                }
            ),
            TowerType::Cannon => (
                assets.tower.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::ZERO,
                    range: 4.5,
                }
            ),
            TowerType::Rock => (
                assets.tower.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.75, TimerMode::Repeating),
                    bullet_offset: Vec3::ZERO,
                    range: 4.5,
                }
            )
        }
    }

    fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Lazer => (
                assets.bullet.clone(),
                Bullet {
                    direction,
                    speed: 10.5,
                }
            ),
            TowerType::Cannon => (
                assets.bullet.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                }
            ),
            TowerType::Rock => (
                assets.bullet.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                }
            )
        }
    }
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .register_inspectable::<TowerType>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(tower_shooting)
                    .with_system(build_tower)
            )
        ;
    }
}


fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .filter(|target_transform| {
                    Vec3::distance(target_transform.translation(), bullet_spawn) < tower.range
                })
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                let (model, bullet) = tower_type.get_bullet(direction, &assets);
                commands.entity(tower_ent)
                    .with_children(|commands| {
                        commands.spawn(SceneBundle {
                            scene: model,
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                            .insert(Lifetime {
                                timer: Timer::from_seconds(0.5, TimerMode::Once) // Bullet lifetime
                            })
                            .insert(bullet)
                            .insert(Name::new("Bullet"))
                            .insert(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)));
                        ;
                    });
            }
        }
    }
}

// Legacy code
fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<GameAssets>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.selected() {
                //commands.entity(entity).despawn_recursive();
                //spawn_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (ts, tower) = tower_type.get_tower(assets);
    info!("Spawning {:?} tower", tower_type);
    commands
        .spawn(SpatialBundle::from_transform(
            Transform::from_translation(position)
        ))
        .insert(Name::new(format!("{:?}_tower", tower_type)))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.pedestal.clone(),
                transform: Transform::from_xyz(0.0, -0.9, 0.0),
                ..default()
            })
                .insert(Name::new("Pedestal"));
            commands.spawn(SceneBundle {
                scene: ts,
                transform: Transform::from_xyz(0.0, -1., 0.0),
                ..default()
            })
                .insert(Name::new("Tower base"));
        }).id()
}