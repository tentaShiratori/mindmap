use crate::model::backend::*;
use crate::model::backend_repository::BackendRepository;
use crate::model::local_backend::{LocalBackend, LocalBackendBackendId, LocalBackendPath};
use crate::model::local_backend_repository::LocalBackendRepository;
use anyhow::Result;

use uuid::Uuid;

use shaku::{Component, Interface};
use std::sync::Arc;

pub struct AddLocalBackendArgs {
    pub name: String,
    pub path: String,
}

pub trait AddLocalBackendUsecase: Interface {
    fn exec(&self, args: AddLocalBackendArgs) -> Result<String>;
}

#[derive(Component)]
#[shaku(interface = AddLocalBackendUsecase)]
pub struct AddLocalBackendUsecaseImpl {
    #[shaku(inject)]
    backend_dao: Arc<dyn BackendRepository>,
    #[shaku(inject)]
    local_backend_dao: Arc<dyn LocalBackendRepository>,
}

impl AddLocalBackendUsecase for AddLocalBackendUsecaseImpl {
    fn exec(&self, args: AddLocalBackendArgs) -> Result<String> {
        let id = Uuid::new_v4();
        let mut data = self.backend_dao.load()?;
        data.push(Backend {
            id: BackendId(id.to_string()),
            name: BackendName(args.name),
            r#type: BackendType::Local,
        });
        self.backend_dao.save(&data)?;

        let mut data = self.local_backend_dao.load()?;
        data.push(LocalBackend {
            backend_id: LocalBackendBackendId(id.to_string()),
            path: LocalBackendPath(args.path),
        });
        self.local_backend_dao.save(&data)?;

        Ok(id.to_string())
    }
}
