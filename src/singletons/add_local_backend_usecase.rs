use crate::app_module::AppModule;

use crate::extends::ui::Return;
use crate::model::local_backend::LocalBackend;
use crate::{model::backend::*, ui, ui::AppWindow};
use anyhow::Result;
use slint::*;

use uuid::Uuid;

use super::dao::Dao;

pub struct AddLocalBackendUsecase<T, U>
where
    T: Dao<Vec<Backend>>,
    U: Dao<Vec<LocalBackend>>,
{
    backend_dao: T,
    local_backend_dao: U,
}

impl<T, U> AddLocalBackendUsecase<T, U>
where
    T: Dao<Vec<Backend>>,
    U: Dao<Vec<LocalBackend>>,
{
    pub fn new(backend_dao: T, local_backend_dao: U) -> AddLocalBackendUsecase<T, U> {
        AddLocalBackendUsecase {
            backend_dao: backend_dao,
            local_backend_dao: local_backend_dao,
        }
    }

    fn exec(&self, args: ui::AddLocalBackendArgs) -> Result<String> {
        let id = Uuid::new_v4();
        let mut data = self.backend_dao.load()?;
        data.push(Backend {
            id: id.to_string(),
            name: args.name.to_string(),
            r#type: ui::BackendType::Local,
        });
        self.backend_dao.save(&data)?;

        let mut data = self.local_backend_dao.load()?;
        data.push(LocalBackend {
            backend_id: id.to_string(),
            path: args.path.to_string(),
        });
        self.local_backend_dao.save(&data)?;

        Ok((id.to_string()))
    }
}

pub fn setup(window: &AppWindow, module: &'static AppModule) {
    let repository = window.global::<ui::AddLocalBackendUsecase>();
    repository.on_exec(|args| {
        let result = module.add_local_backend_usecase.exec(args);
        Return::from(result).into_error()
    });
}
