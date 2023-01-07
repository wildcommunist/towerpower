use bevy::prelude::*;
use crate::gameplay::GameMap;
use crate::states::GameState;
use crate::target::TargetDeathEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(spawn_player)
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

    pub fn set_funds(&mut self, amount: u32) -> u32 {
        self.money = amount;
        self.money
    }

    pub fn set_lives(&mut self, amount: u32) -> u32 {
        self.lives = amount;
        self.lives
    }

    pub fn add_funds(&mut self, amount: u32) -> Option<u32> {
        if let Some(new_bal) = self.money.checked_add(amount) {
            self.money = new_bal;
            return Some(self.money);
        }
        None
    }

    pub fn damage(&mut self, amount: u32) -> Option<u32> {
        println!("Subtracting {} lives. Current: {}", amount, &self.lives);
        if let Some(new_health) = self.lives.checked_sub(amount) {
            self.lives = new_health;
            if new_health == 0 {
                // player dies
                //TODO: maybe handle death logic in a different way
                return None;
            }
            return Some(self.lives);
        }
        None
    }
}

fn spawn_player(
    mut commands: Commands,
    map: Res<GameMap>,
) {
    commands.spawn((Player { money: map.starting_funds, lives: map.starting_lives }, Name::new("Player")));
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