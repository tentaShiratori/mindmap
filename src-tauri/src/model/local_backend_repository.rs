use super::local_backend::LocalBackend;
use anyhow::Result;
use shaku::Interface;

pub trait LocalBackendRepository: Interface {
    fn load(&self) -> Result<Vec<LocalBackend>>;
    fn save(&self, data: &Vec<LocalBackend>) -> Result<()>;
}
