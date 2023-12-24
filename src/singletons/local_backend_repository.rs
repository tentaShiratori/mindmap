use crate::dao::json::JsonDao;
use crate::extends::ui::Return;
use crate::{
    lib::*,
    model::local_backend::*,
    ui,
    ui::{AppWindow, Local},
};
use anyhow::Result;
use slint::*;

use super::dao::Dao;

struct LocalBackendRepository<T>
where
    T: Dao<Vec<LocalBackend>>,
{
    dao: T,
}

impl<T> LocalBackendRepository<T>
where
    T: Dao<Vec<LocalBackend>>,
{
    fn new(dao: T) -> LocalBackendRepository<T> {
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

fn json_dao() -> JsonDao<Vec<LocalBackend>> {
    JsonDao {
        path: dir::Dir::new().data().join("local_backend.json"),
        default_data: vec![],
    }
}

pub fn setup(window: &AppWindow) {
    window
        .global::<ui::LocalBackendRepository>()
        .on_edit(|local| {
            let local_backend_repository = LocalBackendRepository::new(json_dao());
            let result = local_backend_repository.edit(local);

            Return::from(result).into_error()
        });
}
