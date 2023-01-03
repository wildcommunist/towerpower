use std::f32::consts::PI;
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::time::Timer;
use bevy::utils::FloatOrd;
use crate::game_assets::GameAssets;
use crate::target::{Health, Target};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub fn tower_shooting(
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
                                timer: Timer::from_seconds(2.5, TimerMode::Once)
                            })
                            .insert(Bullet {
                                direction,
                                speed: 2.5,
                            })
                            .insert(Name::new("Bullet"));
                    });
            }
        }
    }
}

pub fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (e, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

pub fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}