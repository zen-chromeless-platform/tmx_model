use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Template {
    pub tile_set: Option<crate::tile_set::TileSet>,
    pub object: Box<crate::object::Object>,
}
