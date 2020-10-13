//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use ::{std::ops::{Deref, DerefMut}, serde::{Deserialize, Serialize}};

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
    Polygon(Points),
    PolyLine(Points),
    Text(Text),
}

#[derive(Deserialize, Serialize)]
pub struct Points(Vec<(f64, f64)>);

impl Deref for Points {
    type Target = [(f64, f64)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Points {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deserialize, Serialize)]
pub struct Text {
    pub font_family: String,
    pub pixel_size: i64,
    pub bold: bool,
    pub color: String,
    pub h_align: HorizontalAlignment,
    pub italic: bool,
    pub kerning: bool,
    pub strikeout: bool,
    pub text: String,
    pub underline: bool,
    pub v_align: VerticalAlignment,
    pub wrap: bool,
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
