//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use serde::{Deserialize, Serialize};

/// xml not support
#[derive(Deserialize, Serialize)]
pub enum Data {
    Base64 {
        compression: Compression,
        data: String,
    },
    Csv {
        data: Vec<crate::GlobalTileId>,
    },
}

#[derive(Deserialize, Serialize)]
pub enum Compression {
    None,
    Gzip,
    Zlib,
    Zstd,
}

impl Default for Compression {
    fn default() -> Self {
        Self::None
    }
}
