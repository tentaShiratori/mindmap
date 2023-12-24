use crate::app_module::AppModule;

use crate::extends::ui::Return;
use crate::{
    model::local_backend::*,
    ui,
    ui::{AppWindow, Local},
};
use anyhow::Result;
use slint::*;

use super::dao::Dao;

pub struct LocalBackendRepository<T>
where
    T: Dao<Vec<LocalBackend>>,
{
    dao: T,
}

impl<T> LocalBackendRepository<T>
where
    T: Dao<Vec<LocalBackend>>,
{
    pub fn new(dao: T) -> LocalBackendRepository<T> {
        LocalBackendRepository { dao: dao }
    }

    fn edit(&self, local: Local) -> Result<()> {
        let mut data = self.dao.load()?;
        data.push(LocalBackend {
            backend_id: local.backend_id.to_string(),
            path: local.path.to_string(),
        });
        self.dao.save(&data)?;
        Ok(())
    }
}

pub fn setup(window: &AppWindow, module: &'static AppModule) {
    window
        .global::<ui::LocalBackendRepository>()
        .on_edit(|local| {
            let result = module.local_backend_repository.edit(local);

            Return::from(result).into_error()
        });
}
