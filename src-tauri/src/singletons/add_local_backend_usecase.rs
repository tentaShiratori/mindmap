use crate::model::backend::*;
use crate::model::local_backend::{LocalBackend, LocalBackendBackendId, LocalBackendPath};
use anyhow::Result;

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

pub struct AddLocalBackendArgs {
    id: String,
    name: String,
    path: String,
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
