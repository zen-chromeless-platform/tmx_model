//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub id: crate::ObjectId,
    pub name: String,
    pub r#type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub gid: crate::GlobalTileId,
    pub visible: bool,
    pub shape: Shape,
    pub template: Option<crate::template::Template>,
    pub properties: Option<crate::properties::Properties>,
}

#[derive(Deserialize, Serialize)]
pub enum Shape {
    Rectangle,
    Ellipse,
    Point,
    Polygon {
        points: Vec<(f64, f64)>,
    },
    PolyLine {
        points: Vec<(f64, f64)>,
    },
    Text {
        font_family: String,
        pixel_size: i64,
        bold: bool,
        color: String,
        h_align: HorizontalAlignment,
        italic: bool,
        kerning: bool,
        strikeout: bool,
        text: String,
        underline: bool,
        v_align: VerticalAlignment,
        wrap: bool,
    },
}

#[derive(Deserialize, Serialize)]
pub enum HorizontalAlignment {
    Center,
    Right,
    Justify,
    Left,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Deserialize, Serialize)]
pub enum VerticalAlignment {
    Center,
    Bottom,
    Top,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        Self::Top
    }
}

// #[derive(Deserialize, Serialize)]
// pub struct FontFamily(String);
//
// impl From<String> for FontFamily{
//     fn from(inner: String) -> Self {
//         Self(inner)
//     }
// }
//
// impl Default for FontFamily {
//     fn default() -> Self {
//         "sans-serif".to_owned().into()
//     }
// }
