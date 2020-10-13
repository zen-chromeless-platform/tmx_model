//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Layer {
    TileLayer(TileLayer),
    ObjectGroup(ObjectGroup),
    ImageLayer(ImageLayer),
    Group(Group),
}

#[derive(Deserialize, Serialize)]
pub struct TileLayer {
    pub id: crate::LayerId,
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub opacity: f64,
    pub visible: bool,
    pub tint_color: Option<String>,
    pub offset_x: f64,
    pub offset_y: f64,
    pub properties: Option<crate::properties::Properties>,
    pub chunks: Option<Vec<crate::chunk::Chunk>>,
    pub data: Option<crate::data::Data>,
}

#[derive(Deserialize, Serialize)]
pub struct ObjectGroup {
    pub id: crate::LayerId,
    pub name: String,
    pub color: String,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub opacity: f64,
    pub visible: bool,
    pub tint_color: Option<String>,
    pub offset_x: f64,
    pub offset_y: f64,
    pub draw_order: DrawOrder,
    pub objects: Option<Vec<crate::object::Object>>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub enum DrawOrder {
    TopDown,
    Index,
}

impl Default for DrawOrder {
    fn default() -> Self {
        Self::TopDown
    }
}

#[derive(Deserialize, Serialize)]
pub struct ImageLayer {
    pub id: crate::LayerId,
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub width: i64,
    pub height: i64,
    pub opacity: f64,
    pub visible: bool,
    pub tint_color: Option<String>,
    pub transparent_color: Option<String>,
    pub image: String,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    pub id: crate::LayerId,
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub visible: bool,
    pub tint_color: Option<String>,
    pub layers: Vec<Layer>,
    pub properties: Option<crate::properties::Properties>,
}
