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

    pub fn spend_funds(&mut self, amount: u32) -> Option<u32> {
        if let Some(new_bal) = self.money.checked_sub(amount) {
            self.money = new_bal;
            return Some(self.money);
        }
        None
    }

    pub fn add_funds(&mut self, amount: u32) -> Option<u32> {
        if let Some(new_bal) = self.money.checked_add(amount) {
            self.money = new_bal;
            return Some(self.money);
        }
        None
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((Player { money: 1, lives: 5 }, Name::new("Player")));
}

fn give_money_on_kill(
    mut player: Query<&mut Player>,
    mut death_note_events: EventReader<TargetDeathEvent>,
) {
    let mut player = player.single_mut();
    for _event in death_note_events.iter() {
        player.add_funds(1).expect("Player overflow error on funds add");
        info!("Kill! Money: {}", player.money);
    }
}