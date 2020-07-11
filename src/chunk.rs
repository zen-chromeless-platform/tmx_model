use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Chunk {
    pub height: i64,
    pub width: i64,
    pub x: i64,
    pub y: i64,
    pub data: crate::data::Data,
}
