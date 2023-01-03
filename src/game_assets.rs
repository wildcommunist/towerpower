use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameAssets {
    pub bullet: Handle<Scene>,
    pub mob_spawn_delay: Timer,
}