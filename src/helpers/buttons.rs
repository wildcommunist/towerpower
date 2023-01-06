use bevy::prelude::*;
use crate::game_assets::GameAssets;

pub fn spawn_button(
    commands: &mut Commands,
    assets: &GameAssets,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(100.)),
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
                            font_size: 32.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
        }).id()
}