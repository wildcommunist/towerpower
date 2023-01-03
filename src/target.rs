use bevy::prelude::*;
use crate::game_assets::GameAssets;

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

pub fn move_targets(
    mut targets: Query<(&Target, &mut Transform), (With<Health>, With<Movable>)>,
    time: Res<Time>,
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

pub fn spawn_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<GameAssets>,
    time: Res<Time>,
) {
    assets.mob_spawn_delay.tick(time.delta());
    if assets.mob_spawn_delay.just_finished() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
            .insert(Movable)
            .insert(Target { speed: 0.3 })
            .insert(Health { value: 3 })
            .insert(Name::new("Target"));
    }
}

pub fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health), With<Target>>,
) {
    for (target, health) in &targets {
        if health.value <= 0 {
            commands.entity(target).despawn_recursive();
        }
    }
}