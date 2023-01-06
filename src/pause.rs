use bevy::app::AppExit;
use bevy::prelude::*;
use crate::game_assets::GameAssets;
use crate::helpers::spawn_button;
use crate::states::GameState;

pub struct PauseGamePlugin;

impl Plugin for PauseGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Pause)
                    .with_system(setup_ui)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Pause)
                    .with_system(update)
                    .with_system(process_keyboard_input)
                    .with_system(exit_button_click)
                    .with_system(resume_button_click)
            )
        ;
    }
}

#[derive(Component)]
pub struct PauseUiRoot;

#[derive(Component)]
pub struct ResumeGameButton;

#[derive(Component)]
pub struct ExitGameButton;

fn setup_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let exit_button = spawn_button(&mut commands, &assets, "Quit Game", Color::MIDNIGHT_BLUE);
    commands.entity(exit_button).insert(ExitGameButton);

    let resume_button = spawn_button(&mut commands, &assets, "Resume Game", Color::MIDNIGHT_BLUE);
    commands.entity(resume_button).insert(ResumeGameButton);

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
        ..default()
    }).insert((Name::new("Pause_ui_root"), PauseUiRoot))
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::bottom(Val::Percent(20.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        "- Game Paused -",
                        TextStyle {
                            font: assets.game_font.clone(),
                            font_size: 96.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                    ),
                    ..default()
                });
        })
        .add_child(resume_button)
        .add_child(exit_button)
    ;
}

fn update() {}

fn process_keyboard_input(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut keyboard: ResMut<Input<KeyCode>>,
    entity: Query<Entity, With<PauseUiRoot>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        let ui_root = entity.single();
        commands.entity(ui_root).despawn_recursive();
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}

fn resume_button_click(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    interactions: Query<&Interaction, (With<ResumeGameButton>, Changed<Interaction>)>,
    entity: Query<Entity, With<PauseUiRoot>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let ui_root = entity.single();
            commands.entity(ui_root).despawn_recursive();
            game_state.pop().unwrap();
        }
    }
}

fn exit_button_click(
    interactions: Query<&Interaction, (With<ExitGameButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}