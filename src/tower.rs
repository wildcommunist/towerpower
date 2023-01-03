use bevy::prelude::*;
use bevy::time::Timer;
use bevy::utils::FloatOrd;
use crate::bullet::{Bullet, Lifetime};
use crate::game_assets::GameAssets;
use crate::target::{ Target};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .add_system(tower_shooting)
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
                                timer: Timer::from_seconds(2.5, TimerMode::Once)
                            })
                            .insert(Bullet {
                                direction,
                                speed: 2.9,
                            })
                            .insert(Name::new("Bullet"));
                    });
            }
        }
    }
}