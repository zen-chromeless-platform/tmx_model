//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use ::{
    serde::{Deserialize, Serialize},
    std::ops::{Deref, DerefMut},
};

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
    pub gid: Option<crate::GlobalTileId>,
    pub visible: bool,
    pub shape: Shape,
    pub template: Option<crate::template::Template>,
    pub properties: Option<crate::properties::Properties>,
}

impl Object {
    pub fn new(id: crate::ObjectId) -> Self {
        Self {
            id,
            visible: true,
            shape: Shape::Rectangle,
            name: Default::default(),
            r#type: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            rotation: Default::default(),
            gid: Default::default(),
            template: Default::default(),
            properties: Default::default(),
        }
    }
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

impl From<Vec<(f64, f64)>> for Points {
    fn from(inner: Vec<(f64, f64)>) -> Self {
        Self(inner)
    }
}

impl Points {
    pub fn new() -> Self {
        Self::from(Vec::new())
    }
}

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

impl Default for Text {
    fn default() -> Self {
        Self {
            font_family: "sans-serif".to_owned(),
            pixel_size: 16,
            color: "#000000".to_owned(),
            kerning: true,
            ..Default::default()
        }
    }
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }
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
