use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalBackendBackendId(pub String);

#[derive(Serialize, Deserialize)]
pub struct LocalBackendPath(pub String);

#[derive(Serialize, Deserialize)]
pub struct LocalBackend {
    pub backend_id: LocalBackendBackendId,
    pub path: LocalBackendPath,
}
