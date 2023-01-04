use bevy::prelude::*;
use crate::game_assets::GameAssets;
use crate::physics::PhysicsBundle;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Target>()
            .register_type::<Health>()
            .add_system(spawn_targets)
            .add_system(move_targets)
            .add_system(target_death)
        ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component)]
pub struct Movable;

fn move_targets(
    mut targets: Query<(&Target, &mut Transform), (With<Health>, With<Movable>)>,
    time: Res<Time>,
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn spawn_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<GameAssets>,
    time: Res<Time>,
) {
    assets.mob_spawn_delay.tick(time.delta());
    if assets.mob_spawn_delay.just_finished() {
        commands.spawn(SceneBundle {
            scene: assets.enemy.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
            .insert(Movable)
            .insert(Target { speed: 1.4 })
            .insert(Health { value: 4 })
            .insert(PhysicsBundle::moving_entity(Vec3::new(0.24, 0.24, 0.05)))
            .insert(Name::new("Target"));
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health), With<Target>>,
) {
    for (target, health) in &targets {
        if health.value <= 0 {
            commands.entity(target).despawn_recursive();
        }
    }
}