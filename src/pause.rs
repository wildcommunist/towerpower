use bevy::prelude::*;
use crate::game_assets::GameAssets;
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
            )
        ;
    }
}

#[derive(Component)]
pub struct PauseUiRoot;

fn setup_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            align_self: AlignSelf::FlexStart,
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