use crate::{
    model::{backend::Backend, backend_repository::BackendRepository},
    singletons::dao::Dao,
};

use super::json_dao::JsonDao;
use anyhow::Result;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = BackendRepository)]
pub struct BackendJsonDao {
    json_dao: JsonDao<Vec<Backend>>,
}

impl BackendRepository for BackendJsonDao {
    fn load(&self) -> Result<Vec<Backend>> {
        self.json_dao.load()
    }

    fn save(&self, data: &Vec<Backend>) -> Result<()> {
        self.json_dao.save(data)
    }
}
