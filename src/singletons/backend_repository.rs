use crate::app_module::AppModule;

use crate::extends::ui::Return;
use crate::{
    model::backend::*,
    ui,
    ui::{AppWindow, BackendDraft},
};
use anyhow::Result;
use slint::*;

use uuid::Uuid;

use super::dao::Dao;

pub struct BackendRepository<T>
where
    T: Dao<Vec<Backend>>,
{
    dao: T,
}

impl From<Backend> for ui::Backend {
    fn from(value: Backend) -> Self {
        ui::Backend {
            id: value.id.into(),
            name: value.name.into(),
            r#type: value.r#type,
        }
    }
}

impl<T> BackendRepository<T>
where
    T: Dao<Vec<Backend>>,
{
    pub fn new(dao: T) -> BackendRepository<T> {
        BackendRepository { dao: dao }
    }

    fn add(&self, draft: BackendDraft) -> Result<String> {
        let id = Uuid::new_v4();
        let mut data = self.dao.load()?;
        data.push(Backend {
            id: id.to_string(),
            name: draft.name.to_string(),
            r#type: draft.r#type,
        });
        self.dao.save(&data)?;
        Ok(id.to_string())
    }

    fn get_all(&self) -> Vec<ui::Backend> {
        self.dao
            .load()
            .unwrap_or(vec![])
            .into_iter()
            .map(|b| b.into())
            .collect()
    }
}

pub fn setup(window: &AppWindow, module: &'static AppModule) {
    let repository = window.global::<ui::BackendRepository>();
    repository.on_add(|draft| {
        let result = module.backend_repository.add(draft);
        Return::from(result).into_tuple().into()
    });
    repository.on_getAll(|| module.backend_repository.get_all().as_slice().into())
}
