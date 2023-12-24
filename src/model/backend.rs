use crate::ui::BackendType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Backend {
    pub id: String,
    pub name: String,
    pub r#type: BackendType,
}
