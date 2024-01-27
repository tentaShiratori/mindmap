use crate::{
    lib::dir::Dir,
    model::{local_backend::LocalBackend, local_backend_repository::LocalBackendRepository},
    singletons::dao::Dao,
};

use super::json_dao::JsonDao;
use anyhow::Result;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = LocalBackendRepository)]

pub struct LocalBackendJsonDao {
    #[shaku(default = JsonDao {
        path: Dir::new().data().join("local_backend.json"),
        default_data: vec![],
    })]
    json_dao: JsonDao<Vec<LocalBackend>>,
}

impl LocalBackendRepository for LocalBackendJsonDao {
    fn load(&self) -> Result<Vec<LocalBackend>> {
        self.json_dao.load()
    }

    fn save(&self, data: &Vec<LocalBackend>) -> Result<()> {
        self.json_dao.save(data)
    }
}
