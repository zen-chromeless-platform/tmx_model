//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TileSet {
    pub first_gid: crate::GlobalTileId,
    pub source: String,
    pub name: String,
    pub tile_width: i64,
    pub tile_height: i64,
    pub tile_count: i64,
    pub tile_offset: Option<TileOffset>,
    pub tiles: Option<Vec<Tile>>,
    pub spacing: i64,
    pub margin: i64,
    pub columns: i64,
    pub object_alignment: ObjectAlignment,
    pub image: String,
    pub image_height: i64,
    pub image_width: i64,
    pub background_color: Option<String>,
    pub transparent_color: Option<String>,
    pub grid: Option<Grid>,
    /// XXX: `terraintypes`を省略してよいか？
    pub terrains: Option<Vec<Terrain>>,
    pub wang_sets: Option<Vec<WangSet>>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub enum ObjectAlignment {
    Unspecified,
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

/// `Unspecified` by default
impl Default for ObjectAlignment {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[derive(Deserialize, Serialize)]
pub struct TileOffset {
    pub x: i64,
    pub y: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Grid {
    pub orientation: Orientation,
    pub width: i64,
    pub height: i64,
}

#[derive(Deserialize, Serialize)]
pub enum Orientation {
    Orthogonal,
    Isometric,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Orthogonal
    }
}

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub id: crate::LocalTileId,
    pub r#type: Option<String>,
    pub terrain: Option<Vec<usize>>,
    pub probability: Option<f64>,
    pub image: Option<String>,
    pub image_height: i64,
    pub image_width: i64,
    pub object_group: Option<crate::layer::ObjectGroup>,
    pub animation: Option<Animation>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub struct Animation(Vec<Frame>);

#[derive(Deserialize, Serialize)]
pub struct Frame {
    pub tile_id: crate::LocalTileId,
    pub duration: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Terrain {
    pub name: String,
    pub tile: crate::LocalTileId,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub struct WangSet {
    pub name: String,
    pub tile: crate::LocalTileId,
    /// TODO: Can contain up to 15
    pub corner_colors: Vec<WangColor>,
    /// TODO:  Can contain up to 15
    pub edge_colors: Vec<WangColor>,
    pub wang_tiles: Vec<WangTile>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub struct WangColor {
    pub name: String,
    pub color: String,
    pub tile: crate::LocalTileId,
    pub probability: f64,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub struct WangTile {
    pub tile_id: crate::LocalTileId,
    pub wang_id: WangId,
    pub d_flip: bool,
    pub h_flip: bool,
    pub v_flip: bool,
}

#[derive(Deserialize, Serialize)]
pub struct WangId([u8; 8]);

impl std::ops::Deref for WangId {
    type Target = [u8; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
