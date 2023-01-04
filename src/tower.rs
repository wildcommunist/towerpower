use bevy::prelude::*;
use bevy::time::Timer;
use bevy::utils::FloatOrd;
use bevy_mod_picking::Selection;
use crate::bullet::{Bullet, Lifetime};
use crate::game_assets::GameAssets;
use crate::physics::PhysicsBundle;
use crate::target::{Target};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .add_system(tower_shooting)
            .add_system(build_tower)
        ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;
            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_ent)
                    .with_children(|commands| {
                        commands.spawn(SceneBundle {
                            scene: assets.bullet.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                            .insert(Lifetime {
                                timer: Timer::from_seconds(0.5, TimerMode::Once)
                            })
                            .insert(Bullet {
                                direction,
                                speed: 10.,
                            })
                            .insert(Name::new("Bullet"))
                            .insert(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)));
                        ;
                    });
            }
        }
    }
}

fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<GameAssets>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.selected() {
                commands.entity(entity).despawn_recursive();
                spawn_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
}

fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
) -> Entity {
    commands
        .spawn(SpatialBundle::from_transform(
            Transform::from_translation(position)
        ))
        .insert(Name::new("Zap Tower"))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0., 0.0),
        })
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.pedestal.clone(),
                transform: Transform::from_xyz(0.0, -0.9, 0.0),
                ..default()
            })
                .insert(Name::new("Pedestal"));
            commands.spawn(SceneBundle {
                scene: assets.tower.clone(),
                transform: Transform::from_xyz(0.0, -1., 0.0),
                ..default()
            })
                .insert(Name::new("Tower base"));
        }).id()
}