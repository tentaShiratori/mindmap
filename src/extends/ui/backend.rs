use crate::{model::backend::*, ui};

impl From<Backend> for ui::Backend {
    fn from(value: Backend) -> Self {
        ui::Backend {
            id: value.id.into(),
            name: value.name.into(),
            r#type: value.r#type,
        }
    }
}
