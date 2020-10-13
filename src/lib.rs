//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
//! currently only supported Tiled 1.4

pub mod chunk;
pub mod data;
pub mod editor_settings;
pub mod layer;
pub mod map;
pub mod object;
pub mod properties;
pub mod template;
pub mod tile_set;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ObjectId(i64);

#[derive(Deserialize, Serialize)]
pub struct LayerId(i64);

/// a.k.a gid
#[derive(Deserialize, Serialize)]
pub struct GlobalTileId(i64);

#[derive(Deserialize, Serialize)]
pub struct LocalTileId(i64);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
