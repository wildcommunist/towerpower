use bevy::app::AppExit;
use bevy::prelude::*;
use git2::{Repository};
use crate::game_assets::GameAssets;
use crate::helpers::spawn_button;
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
                        margin: UiRect::bottom(Val::Percent(20.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Tower Defense",
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
        .add_child(exit_button);

    let repo = match Repository::open(".") {
        Ok(r) => {
            let head = r.head().unwrap();
            let commit = head.peel_to_commit().unwrap();
            let hash = commit.id();
            format!("{}", hash)
        }
        Err(_) => String::from("N/A")
    };

    commands
        .spawn(TextBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_content: AlignContent::FlexStart,
                //align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Percent(5.0)),
                ..default()
            },
            text: Text::from_section(
                format!("Commit: {}", repo),
                TextStyle {
                    font: assets.game_font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        });
}

fn start_button_click(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartGameButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::Gameplay).unwrap();
            mouse_input.clear(); // This prevents the "click" from propogating and selecting a tower on screens
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