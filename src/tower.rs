use std::f32::consts::PI;
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::time::Timer;
use crate::game_assets::GameAssets;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub(crate) shooting_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub(crate) timer: Timer,
}

pub fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            // The cooldown is done, yeet it
            let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands.spawn(SceneBundle {
                scene: assets.bullet.clone(),
                transform: spawn_transform,
                ..default()
            })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once)
                })
                .insert(Name::new("Bullet"));
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