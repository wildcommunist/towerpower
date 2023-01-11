use bevy::math::Vec3Swizzles;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use crate::game_assets::GameAssets;
use crate::gameplay::GameMap;
use crate::physics::PhysicsBundle;
use crate::player::Player;
use crate::states::GameState;

pub struct TargetDeathEvent;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Target>()
            .register_type::<Health>()

            .add_event::<TargetDeathEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay)
                .with_system(show_waypoints)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(spawn_targets)
                    .with_system(move_targets)
                    .with_system(target_death)
                    .with_system(check_waypoints.after(move_targets))
            )
        ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component)]
pub struct Waypoints;

#[derive(Component)]
pub struct Movable;

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform), (With<Health>, With<Movable>)>,
    path: Res<GameMap>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        if delta_target.length() > delta {

            // we are still some way off the target, look at the target and yeet yourself that way
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            // we have reached the target, increment the index
            // TODO: Reached end of path, maybe emit an event that we are done
            target.path_index += 1;
        }
    }
}

fn show_waypoints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    path: Res<GameMap>,
) {
    commands.spawn(SpatialBundle {
        ..default()
    }).with_children(|commands| {
        for wp in &path.waypoints {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                material: materials.add(Color::rgba(1., 0.063, 0.941, 0.65).into()),
                transform: Transform {
                    translation: Vec3::new(wp.x, 0., wp.y),
                    ..default()
                },
                ..default()
            })
                .insert(NotShadowCaster)
                .insert(Name::new(format!("waypoint_{}_{}", wp.x, wp.y)));
        }
    }).insert(Name::new("waypoints"));
}

fn spawn_targets(
    mut commands: Commands,
    mut assets: ResMut<GameAssets>,
    time: Res<Time>,
    path: Res<GameMap>,
) {
    let spawn = Vec3::new(path.waypoints[0].x, 0.1, path.waypoints[0].y);
    assets.mob_spawn_delay.tick(time.delta());
    if assets.mob_spawn_delay.just_finished() {
        commands.spawn(SceneBundle {
            scene: assets.enemy.clone(),
            transform: Transform::from_xyz(spawn.x, spawn.y, spawn.z),
            ..default()
        })
            .insert(Movable)
            .insert(Target { speed: 1.4, path_index: 0 })
            .insert(Health { value: 4 })
            .insert(PhysicsBundle::moving_entity(Vec3::new(0.24, 0.24, 0.1)))
            .insert(Name::new("Target"));
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health), With<Target>>,
    mut death_note: EventWriter<TargetDeathEvent>,
) {
    for (target, health) in &targets {
        if health.value <= 0 {
            death_note.send(TargetDeathEvent);
            commands.entity(target).despawn_recursive();
        }
    }
}

fn check_waypoints(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<GameMap>,
    mut player: Query<&mut Player>,
    audio: Res<Audio>,
    assets: Res<GameAssets>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            // Maybe do this via an event system
            // we reached the end
            audio.play(assets.enemy_death_sounds.clone());

            commands.entity(entity).despawn_recursive();
            let mut player = player.single_mut();
            if player.damage(1).is_none() {
                // we returned no lives, means we are at 0 or under lives - aka dead
                info!("GAME OVER"); //TODO: Do something better
            }
        }
    }
}