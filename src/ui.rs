use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy_mod_picking::Selection;
use crate::game_assets::GameAssets;
use crate::tower::{spawn_tower, TowerType};

#[derive(Component)]
pub struct TowerUIRoot;


pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(create_ui_on_selection)
            .add_system(tower_button_clicked)
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
    mut commands: Commands,
    selections: Query<(Entity, &Selection, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (interaction, tower_type) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selections {
                if selection.selected() {
                    commands.entity(entity).despawn_recursive();
                    spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                }
            }
        }
    }
}