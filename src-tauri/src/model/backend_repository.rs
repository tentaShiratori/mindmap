use super::backend::Backend;
use anyhow::Result;
use shaku::Interface;

pub trait BackendRepository: Interface {
    fn load(&self) -> Result<Vec<Backend>>;
    fn save(&self, data: &Vec<Backend>) -> Result<()>;
}
