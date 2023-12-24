use crate::{lib::dir::Dir, model::backend::Backend, singletons::dao::Dao};

use super::json_dao::JsonDao;
use anyhow::Result;

pub struct BackendJsonDao {
    json_dao: JsonDao<Vec<Backend>>,
}

impl Dao<Vec<Backend>> for BackendJsonDao {
    fn load(&self) -> Result<Vec<Backend>> {
        self.json_dao.load()
    }

    fn save(&self, data: &Vec<Backend>) -> Result<()> {
        self.json_dao.save(data)
    }
}

impl BackendJsonDao {
    pub fn new() -> BackendJsonDao {
        BackendJsonDao {
            json_dao: JsonDao {
                path: Dir::new().data().join("backend.json"),
                default_data: vec![],
            },
        }
    }
}
