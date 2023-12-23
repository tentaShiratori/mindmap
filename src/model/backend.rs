use crate::ui::BackendType;
use anyhow::Result;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::fmt;

impl Serialize for BackendType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BackendType::Local => serializer.serialize_str("Local"),
        }
    }
}

struct BackendTypeVisitor;

impl<'de> Visitor<'de> for BackendTypeVisitor {
    type Value = BackendType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string either Local")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "Local" => Ok(BackendType::Local),
            _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &"Local")),
        }
    }
}

impl<'de> Deserialize<'de> for BackendType {
    fn deserialize<D>(deserializer: D) -> Result<BackendType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BackendTypeVisitor)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Backend {
    pub id: String,
    pub name: String,
    pub r#type: BackendType,
}
