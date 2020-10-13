//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Map {
    pub version: String,
    pub tiled_version: Option<String>,
    pub orientation: Orientation,
    /// TODO: `currently only supported for orthogonal maps`
    pub render_order: RenderOrder,
    pub compression_level: i64,
    pub width: i64,
    pub height: i64,
    pub tile_width: i64,
    pub tile_height: i64,
    pub background_color: Option<String>,
    pub next_layer_id: crate::LayerId,
    pub next_object_id: crate::ObjectId,
    pub infinite: bool,
    pub tile_sets: Option<Vec<crate::tile_set::TileSet>>,
    pub layers: Option<Vec<crate::layer::Layer>>,
    pub editor_settings: Option<crate::editor_settings::EditorSettings>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered {
        stagger_axis: StaggerAxis,
        stagger_index: StaggerIndex,
    },
    Hexagonal {
        hex_side_length: i64,
        stagger_axis: StaggerAxis,
        stagger_index: StaggerIndex,
    },
}

#[derive(Deserialize, Serialize)]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl Default for RenderOrder {
    fn default() -> Self {
        Self::RightDown
    }
}

#[derive(Deserialize, Serialize)]
pub enum StaggerAxis {
    X,
    Y,
}

#[derive(Deserialize, Serialize)]
pub enum StaggerIndex {
    Odd,
    Even,
}
