use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Lazer,
    Cannon,
    Rock,
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_ui)
            .add_system(tower_button_clicked)
        ;
    }
}

fn create_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let towers = [TowerType::Lazer, TowerType::Cannon, TowerType::Rock];
    let button_icons: [Handle<Image>; 3] = [
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
            for i in 0..towers.len() {
                commands.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    image: button_icons[i].clone().into(),
                    ..default()
                })
                    .insert(Name::new(format!("Tower_{:?}", towers[i])))
                    .insert(towers[i]);
            }
        });
}

fn tower_button_clicked(
    interactions: Query<(&Interaction, &TowerType), Changed<Interaction>>, // Query will return ONLY changed interactions
) {
    for (interaction, tower_type) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            info!("Spawning: {:?}", tower_type);
        }
    }
}