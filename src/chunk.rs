//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Chunk {
    pub height: i64,
    pub width: i64,
    pub x: i64,
    pub y: i64,
    pub data: crate::data::Data,
}
