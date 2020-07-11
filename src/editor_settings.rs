use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditorSettings {
    pub chunk_size: Option<ChunkSize>,
    pub export: Export,
}

#[derive(Deserialize, Serialize)]
pub struct ChunkSize {
    pub width: i64,
    pub height: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Export {
    pub target: String,
    pub format: String,
}
