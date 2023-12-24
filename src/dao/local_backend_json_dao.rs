use crate::{lib::dir::Dir, model::local_backend::LocalBackend, singletons::dao::Dao};

use super::json_dao::JsonDao;
use anyhow::Result;

pub struct LocalBackendJsonDao {
    json_dao: JsonDao<Vec<LocalBackend>>,
}

impl Dao<Vec<LocalBackend>> for LocalBackendJsonDao {
    fn load(&self) -> Result<Vec<LocalBackend>> {
        self.json_dao.load()
    }

    fn save(&self, data: &Vec<LocalBackend>) -> Result<()> {
        self.json_dao.save(data)
    }
}

impl LocalBackendJsonDao {
    pub fn new() -> LocalBackendJsonDao {
        LocalBackendJsonDao {
            json_dao: JsonDao {
                path: Dir::new().data().join("local_backend.json"),
                default_data: vec![],
            },
        }
    }
}
