use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalBackend {
    pub backend_id: String,
    pub path: String,
}
