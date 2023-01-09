use std::fs;
use bevy::math::Vec2;
use bevy::prelude::*;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::gameplay::GameMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "__header__")]
    pub header: Header,
    pub iid: String,
    pub json_version: String,
    pub app_build_id: i64,
    pub next_uid: i64,
    pub identifier_style: String,
    pub world_layout: String,
    pub world_grid_width: i64,
    pub world_grid_height: i64,
    pub default_level_width: i64,
    pub default_level_height: i64,
    pub default_pivot_x: i64,
    pub default_pivot_y: i64,
    pub default_grid_size: i64,
    pub bg_color: String,
    pub default_level_bg_color: String,
    pub minify_json: bool,
    pub external_levels: bool,
    pub export_tiled: bool,
    pub simplified_export: bool,
    pub image_export_mode: String,
    pub export_level_bg: bool,
    pub png_file_pattern: Value,
    pub backup_on_save: bool,
    pub backup_limit: i64,
    pub level_name_pattern: String,
    pub tutorial_desc: Value,
    pub custom_commands: Vec<Value>,
    pub flags: Vec<Value>,
    pub defs: Defs,
    pub levels: Vec<Level>,
    pub worlds: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub file_type: String,
    pub app: String,
    pub doc: String,
    pub schema: String,
    pub app_author: String,
    pub app_version: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub layers: Vec<Layer>,
    pub entities: Vec<Entity>,
    pub tilesets: Vec<Tileset>,
    pub enums: Vec<Enum>,
    pub external_enums: Vec<Value>,
    pub level_fields: Vec<LevelField>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde(rename = "__type")]
    pub type_field: String,
    pub identifier: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    pub uid: i64,
    pub grid_size: i64,
    pub guide_grid_wid: i64,
    pub guide_grid_hei: i64,
    pub display_opacity: i64,
    pub inactive_opacity: f64,
    pub hide_in_list: bool,
    pub hide_fields_when_inactive: bool,
    pub can_select_when_inactive: bool,
    pub px_offset_x: i64,
    pub px_offset_y: i64,
    pub parallax_factor_x: i64,
    pub parallax_factor_y: i64,
    pub parallax_scaling: bool,
    pub required_tags: Vec<Value>,
    pub excluded_tags: Vec<Value>,
    pub int_grid_values: Vec<IntGridValue>,
    pub auto_rule_groups: Vec<Value>,
    pub auto_source_layer_def_uid: Value,
    pub tileset_def_uid: Value,
    pub tile_pivot_x: i64,
    pub tile_pivot_y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValue {
    pub value: i64,
    pub identifier: String,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub identifier: String,
    pub uid: i64,
    pub tags: Vec<Value>,
    pub width: i64,
    pub height: i64,
    pub resizable_x: bool,
    pub resizable_y: bool,
    pub keep_aspect_ratio: bool,
    pub tile_opacity: i64,
    pub fill_opacity: f64,
    pub line_opacity: i64,
    pub hollow: bool,
    pub color: String,
    pub render_mode: String,
    pub show_name: bool,
    pub tileset_id: i64,
    pub tile_render_mode: String,
    pub tile_rect: TileRect,
    pub nine_slice_borders: Vec<Value>,
    pub max_count: i64,
    pub limit_scope: String,
    pub limit_behavior: String,
    pub pivot_x: i64,
    pub pivot_y: i64,
    pub field_defs: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileRect {
    pub tileset_uid: i64,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    #[serde(rename = "__cWid")]
    pub c_wid: i64,
    #[serde(rename = "__cHei")]
    pub c_hei: i64,
    pub identifier: String,
    pub uid: i64,
    pub rel_path: Value,
    pub embed_atlas: String,
    pub px_wid: i64,
    pub px_hei: i64,
    pub tile_grid_size: i64,
    pub spacing: i64,
    pub padding: i64,
    pub tags: Vec<Value>,
    pub tags_source_enum_uid: Value,
    pub enum_tags: Vec<Value>,
    pub custom_data: Vec<Value>,
    pub saved_selections: Vec<Value>,
    pub cached_pixel_data: CachedPixelData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedPixelData {
    pub opaque_tiles: String,
    pub average_colors: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    pub identifier: String,
    pub uid: i64,
    pub values: Vec<Value>,
    pub icon_tileset_uid: i64,
    pub external_rel_path: Value,
    pub external_file_checksum: Value,
    pub tags: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelField {
    pub identifier: String,
    pub doc: Value,
    #[serde(rename = "__type")]
    pub type_field: String,
    pub uid: i64,
    #[serde(rename = "type")]
    pub type_field2: String,
    pub is_array: bool,
    pub can_be_null: bool,
    pub array_min_length: Value,
    pub array_max_length: Value,
    pub editor_display_mode: String,
    pub editor_display_pos: String,
    pub editor_link_style: String,
    pub editor_always_show: bool,
    pub editor_show_in_world: bool,
    pub editor_cut_long_values: bool,
    pub editor_text_suffix: Value,
    pub editor_text_prefix: Value,
    pub use_for_smart_color: bool,
    pub min: Value,
    pub max: Value,
    pub regex: Value,
    pub accept_file_types: Value,
    pub default_override: DefaultOverride,
    pub text_language_mode: Value,
    pub symmetrical_ref: bool,
    pub auto_chain_ref: bool,
    pub allow_out_of_level_ref: bool,
    pub allowed_refs: String,
    pub allowed_ref_tags: Vec<Value>,
    pub tileset_uid: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultOverride {
    pub id: String,
    pub params: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub identifier: String,
    pub iid: String,
    pub uid: i64,
    pub world_x: i64,
    pub world_y: i64,
    pub world_depth: i64,
    pub px_wid: i64,
    pub px_hei: i64,
    #[serde(rename = "__bgColor")]
    pub bg_color: String,
    #[serde(rename = "bgColor")]
    pub bg_color2: Value,
    pub use_auto_identifier: bool,
    pub bg_rel_path: Value,
    pub bg_pos: Value,
    pub bg_pivot_x: f64,
    pub bg_pivot_y: f64,
    #[serde(rename = "__smartColor")]
    pub smart_color: String,
    #[serde(rename = "__bgPos")]
    pub bg_pos2: Value,
    pub external_rel_path: Value,
    pub field_instances: Vec<FieldInstance>,
    pub layer_instances: Vec<LayerInstance>,
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__value")]
    pub value: i64,
    #[serde(rename = "__type")]
    pub type_field: String,
    #[serde(rename = "__tile")]
    pub tile: Value,
    pub def_uid: i64,
    pub real_editor_values: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__type")]
    pub type_field: String,
    #[serde(rename = "__cWid")]
    pub c_wid: i64,
    #[serde(rename = "__cHei")]
    pub c_hei: i64,
    #[serde(rename = "__gridSize")]
    pub grid_size: i64,
    #[serde(rename = "__opacity")]
    pub opacity: i64,
    #[serde(rename = "__pxTotalOffsetX")]
    pub px_total_offset_x: i64,
    #[serde(rename = "__pxTotalOffsetY")]
    pub px_total_offset_y: i64,
    #[serde(rename = "__tilesetDefUid")]
    pub tileset_def_uid: Value,
    #[serde(rename = "__tilesetRelPath")]
    pub tileset_rel_path: Value,
    pub iid: String,
    pub level_id: i64,
    pub layer_def_uid: i64,
    pub px_offset_x: i64,
    pub px_offset_y: i64,
    pub visible: bool,
    pub optional_rules: Vec<Value>,
    pub int_grid_csv: Vec<i64>,
    pub auto_layer_tiles: Vec<Value>,
    pub seed: i64,
    pub override_tileset_uid: Value,
    pub grid_tiles: Vec<Value>,
    pub entity_instances: Vec<EntityInstance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__grid")]
    pub grid: Vec<i64>,
    #[serde(rename = "__pivot")]
    pub pivot: Vec<i64>,
    #[serde(rename = "__tags")]
    pub tags: Vec<Value>,
    #[serde(rename = "__tile")]
    pub tile: Tile,
    #[serde(rename = "__smartColor")]
    pub smart_color: String,
    pub iid: String,
    pub width: i64,
    pub height: i64,
    pub def_uid: i64,
    pub px: Vec<i64>,
    pub field_instances: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub tileset_uid: i64,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}


impl GameMap {
    pub fn load(level: &str) -> anyhow::Result<Self> {
        info!("loading map: {}", level);
        let data = fs::read_to_string(level)?;
        let root_data: Root = serde_json::from_str(&data)?;
        info!("Loaded map {}", root_data.levels[0].identifier);

        let grid_cell_size = root_data.levels[0].layer_instances[1].grid_size as f32 / 4.0;
        let map_width = root_data.levels[0].layer_instances[1].c_wid as f32;
        let map_height = root_data.levels[0].layer_instances[1].c_hei as f32;

        let mut waypoints: Vec<Vec2> = Vec::new();
        for entity_wp in &root_data.levels[0].layer_instances[0].entity_instances {
            waypoints.push(Vec2::new((entity_wp.grid[0] as f32 * grid_cell_size) + grid_cell_size / 2.0, (entity_wp.grid[1] as f32 * grid_cell_size) + grid_cell_size / 2.0));
        }

        let starting_lives = match root_data.levels[0].field_instances.iter().find(|&v| v.identifier == "starting_lives") {
            None => 0.,
            Some(v) => v.value as f32
        };

        let starting_funds = match root_data.levels[0].field_instances.iter().find(|&v| v.identifier == "starting_funds") {
            None => 0.,
            Some(v) => v.value as f32
        };

        Ok(GameMap {
            starting_lives: starting_lives as u32,
            starting_funds: starting_funds as u32,
            name: root_data.levels[0].identifier.to_string(),
            width: map_width,
            height: map_height,
            waypoints,
            grid_size: grid_cell_size as u32,
        })
    }
}