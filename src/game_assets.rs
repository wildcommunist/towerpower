use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameAssets {
    pub bullet: Handle<Scene>,
    pub pedestal: Handle<Scene>,
    pub tower: Handle<Scene>,
    pub enemy: Handle<Scene>,
    pub mob_spawn_delay: Timer,
    pub game_font: Handle<Font>,
    pub enemy_death_sounds: Handle<AudioSource>,
}