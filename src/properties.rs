//Copyright (c) 2020 - , zen-chromeless-platform All rights reserved.
use ::{
    serde::{Deserialize, Serialize},
    std::{collections::BTreeMap, ops::Deref},
};

#[derive(Deserialize, Serialize)]
pub struct Properties(BTreeMap<String, Property>);

impl Deref for Properties {
    type Target = BTreeMap<String, Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Serialize)]
pub enum Property {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Color(String),
    File(String),
    Object(crate::ObjectId),
}
