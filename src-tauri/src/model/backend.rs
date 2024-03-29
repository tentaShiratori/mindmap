use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BackendType {
    Local,
}

#[derive(Serialize, Deserialize)]
pub struct BackendId(pub String);

#[derive(Serialize, Deserialize)]
pub struct BackendName(pub String);

#[derive(Serialize, Deserialize)]
pub struct Backend {
    pub id: BackendId,
    pub name: BackendName,
    pub r#type: BackendType,
}
