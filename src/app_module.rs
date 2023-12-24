use crate::dao::*;
use crate::singletons::*;

pub struct AppModule {
    pub backend_repository: backend_repository::BackendRepository<backend_json_dao::BackendJsonDao>,
    pub local_backend_repository: local_backend_repository::LocalBackendRepository<
        local_backend_json_dao::LocalBackendJsonDao,
    >,
}

impl AppModule {
    pub fn new() -> AppModule {
        AppModule {
            backend_repository: backend_repository::BackendRepository::new(
                backend_json_dao::BackendJsonDao::new(),
            ),
            local_backend_repository: local_backend_repository::LocalBackendRepository::new(
                local_backend_json_dao::LocalBackendJsonDao::new(),
            ),
        }
    }
}
