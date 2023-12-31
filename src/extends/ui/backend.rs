use crate::{model::backend::*, ui};

impl From<Backend> for ui::Backend {
    fn from(value: Backend) -> Self {
        ui::Backend {
            id: value.id.0.into(),
            name: value.name.0.into(),
            r#type: value.r#type.0,
        }
    }
}
