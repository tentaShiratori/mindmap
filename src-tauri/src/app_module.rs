use crate::dao::*;
use crate::singletons::*;

pub struct AppModule {
    pub add_local_backend_usecase: add_local_backend_usecase::AddLocalBackendUsecase<
        backend_json_dao::BackendJsonDao,
        local_backend_json_dao::LocalBackendJsonDao,
    >,
}

impl AppModule {
    pub fn new() -> AppModule {
        AppModule {
            add_local_backend_usecase: add_local_backend_usecase::AddLocalBackendUsecase::new(
                backend_json_dao::BackendJsonDao::new(),
                local_backend_json_dao::LocalBackendJsonDao::new(),
            ),
        }
    }
}
