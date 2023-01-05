use bevy::app::AppExit;
use bevy::prelude::*;
use crate::game_assets::GameAssets;
use crate::states::GameState;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct EndGameButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(spawn_main_menu)
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(exit_button_click)
                    .with_system(start_button_click)
            )
        ;
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let start_button = spawn_button(&mut commands, &assets, "Start Game", Color::RED);
    commands.entity(start_button).insert(StartGameButton);

    let exit_button = spawn_button(&mut commands, &assets, "Exit", Color::MIDNIGHT_BLUE);
    commands.entity(exit_button).insert(EndGameButton);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Percent(3.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Tower Defense tutorial",
                        TextStyle {
                            font: assets.game_font.clone(),
                            font_size: 96.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
        })
        .add_child(start_button)
        .add_child(exit_button)
    ;
}

fn spawn_button(
    commands: &mut Commands,
    assets: &GameAssets,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(65.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Percent(3.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: assets.game_font.clone(),
                            font_size: 64.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
        }).id()
}

fn start_button_click(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartGameButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::Gameplay).unwrap();
        }
    }
}

fn exit_button_click(
    interactions: Query<&Interaction, (With<EndGameButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}