use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy_mod_picking::Selection;
use crate::game_assets::GameAssets;
use crate::player::Player;
use crate::states::GameState;
use crate::tower::{spawn_tower, TowerType};

#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
    cost: u32,
    affordable: bool,
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(create_ui_on_selection)
                    .with_system(tower_button_clicked)
                    .with_system(update_tower_button_states)
                    .with_system(update_tower_button_states.after(create_ui_on_selection)) // Make sure we update the state after the UI has been created
            )
        ;
    }
}

fn create_ui_on_selection(
    mut commands: Commands,
    assets: Res<AssetServer>,
    selections: Query<&Selection>, //bevy selection crate
    root: Query<Entity, With<TowerUIRoot>>, // we need to get our ui root so we can (de)spawn it
) {
    let at_least_one_selected = selections.iter().any(|s| s.selected());
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                // we dont have anything selected BUT have an active ui root, despawn
                commands.entity(root).despawn_recursive();
            }
        }
        Err(QuerySingleError::NoEntities(..)) => {
            // we have something selected but NO UI root (towers) present, spawn it
            if at_least_one_selected {
                create_ui(&mut commands, &assets);
            }
        }
        _ => unreachable!("Too many ui tower roots"),
    }
}

fn create_ui(
    commands: &mut Commands,
    assets: &AssetServer,
) {
    let tower_types = [TowerType::Lazer, TowerType::Cannon, TowerType::Rock];
    let tower_costs = [1, 2, 5];

    let tower_icons: [Handle<Image>; 3] = [
        assets.load("images/rock_tower_icon.png"),
        assets.load("images/rock_tower_icon.png"),
        assets.load("images/rock_tower_icon.png"),
    ];
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TowerUIRoot)
        .insert(Name::new("UI_Root"))
        .with_children(|commands| {
            for i in 0..tower_types.len() {
                commands.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    image: tower_icons[i].clone().into(),
                    ..default()
                })
                    .insert(TowerButtonState {
                        cost: tower_costs[i],
                        affordable: false,
                    })
                    .insert(Name::new(format!("Tower_{:?}", tower_types[i])))
                    .insert(tower_types[i]);
            }
        });
}

fn tower_button_clicked(
    interactions: Query<(&Interaction, &TowerType, &TowerButtonState), Changed<Interaction>>, // Query will return ONLY changed interactions
    mut commands: Commands,
    selections: Query<(Entity, &Selection, &Transform)>,
    mut player: Query<&mut Player>,
    assets: Res<GameAssets>,
) {
    let mut player = player.single_mut();
    for (interaction, tower_type, button_state) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selections {
                if selection.selected() {
                    if player.get_funds() >= button_state.cost {
                        commands.entity(entity).despawn_recursive();
                        spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                    } else {
                        info!("Cannot afford {:?} tower, it costs {} but only have {}", tower_type,button_state.cost,player.get_funds());
                    }
                }
            }
        }
    }
}

fn update_tower_button_states(
    mut buttons: Query<(&mut BackgroundColor, &mut TowerButtonState)>,
    player: Query<&Player>,
) {
    let player = player.single();

    for (mut button_tint, mut state) in &mut buttons {
        if player.get_funds() >= state.cost {
            state.affordable = true;
            *button_tint = Color::WHITE.into();
        } else {
            state.affordable = true;
            *button_tint = Color::DARK_GRAY.into();
        }
    }
}