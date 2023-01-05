use bevy::prelude::*;
use crate::states::GameState;
use crate::target::TargetDeathEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay).with_system(spawn_player)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(give_money_on_kill)
            )
        ;
    }
}


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    money: u32,
    lives: u32,
}

impl Player {
    pub fn get_funds(&self) -> u32 {
        self.money
    }

    pub fn get_lives(&self) -> u32 {
        self.lives
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((Player { money: 0, lives: 5 }, Name::new("Player")));
}

fn give_money_on_kill(
    mut player: Query<&mut Player>,
    mut death_note_events: EventReader<TargetDeathEvent>,
) {
    let mut player = player.single_mut();
    for _event in death_note_events.iter() {
        player.money += 10;
        info!("Kill! Money: {}", player.money);
    }
}