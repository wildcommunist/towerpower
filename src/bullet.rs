use bevy::prelude::*;
use crate::target::{Health, Target};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_system(move_bullets)
            .add_system(bullet_despawn)
            //.add_system(bullet_collision)
        ;
    }
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

fn bullet_despawn(
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

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
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