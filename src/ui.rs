use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy_mod_picking::Selection;
use crate::game_assets::GameAssets;
use crate::player::Player;
use crate::states::GameState;
use crate::tower::{spawn_tower, TowerType};

#[derive(Component)]
pub struct TowerUiRoot;

#[derive(Component)]
pub struct GameplayUiRoot;

#[derive(Component)]
pub struct MoneyUiElement;

#[derive(Component)]
pub struct LivesUiElement;

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
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(spawn_gameplay_ui)
            )
            .add_system_set(
                SystemSet::on_pause(GameState::Gameplay)
                    .with_system(destruct_ui)
            )
            .add_system_set(
                SystemSet::on_resume(GameState::Gameplay)
                    .with_system(destruct_ui)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(create_ui_on_selection)
                    .with_system(tower_button_clicked)
                    .with_system(process_keyboard_input)
                    .with_system(update_tower_button_states)
                    .with_system(update_tower_button_states.after(create_ui_on_selection)) // Make sure we update the state after the UI has been created
                    .with_system(update_player_ui)
            )
        ;
    }
}

fn destruct_ui(
    mut commands: Commands,
    root: Query<Entity, With<TowerUiRoot>>, // we need to get our ui root so we can (de)spawn it
) {
    for tower_ui_root in &root {
        commands.entity(tower_ui_root).despawn_recursive();
    }
}

fn spawn_gameplay_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart, // Flex start is top left
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
        .insert((Name::new("Game_Ui_Root"), GameplayUiRoot))
        .with_children(|commands| {
            commands
                .spawn(NodeBundle { // This is the row where text components for lives and money will live
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::SpaceBetween, // equal space between items
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        flex_direction: FlexDirection::Row, // which way should items go
                        ..default()
                    },
                    ..default()
                }).with_children(|commands| {
                commands
                    .spawn(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Percent(1.2)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Funds: XX",
                            TextStyle {
                                font: assets.game_font.clone(),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    }).insert(MoneyUiElement);

                commands
                    .spawn(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Percent(1.2)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Lives: XX",
                            TextStyle {
                                font: assets.game_font.clone(),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    }).insert(LivesUiElement);
            });
        })
    ;
}

fn create_ui_on_selection(
    mut commands: Commands,
    assets: Res<AssetServer>,
    selections: Query<&Selection>, //bevy selection crate
    root: Query<Entity, With<TowerUiRoot>>, // we need to get our ui root so we can (de)spawn it
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
        .insert(TowerUiRoot)
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
    audio: Res<Audio>,
) {
    let mut player = player.single_mut();
    for (interaction, tower_type, button_state) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selections {
                if selection.selected() {
                    if player.get_funds() >= button_state.cost {
                        match player.spend_funds(button_state.cost) {
                            None => {
                                warn!("Player balance overflow error");
                            }
                            Some(_) => {
                                audio.play(assets.tower_place_sound.clone());
                                commands.entity(entity).despawn_recursive();
                                spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                            }
                        }
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

fn update_player_ui(
    player: Query<&Player>,
    mut money_ui: Query<&mut Text, (With<MoneyUiElement>, Without<LivesUiElement>)>,
    mut lives_ui: Query<&mut Text, With<LivesUiElement>>,
) {
    let player = player.single();
    let mut money_ui = money_ui.single_mut();
    let mut lives_ui = lives_ui.single_mut();

    *money_ui = Text::from_section(
        format!("Funds: {}", player.get_funds()),
        money_ui.sections[0].style.clone(),
    );

    *lives_ui = Text::from_section(
        format!("Lives: {}", player.get_lives()),
        lives_ui.sections[0].style.clone(),
    );
}

fn process_keyboard_input(
    mut game_state: ResMut<State<GameState>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        game_state.push(GameState::Pause).unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}