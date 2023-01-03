use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub(crate) bullet: Handle<Scene>,
}