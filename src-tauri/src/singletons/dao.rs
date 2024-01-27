use anyhow::Result;

pub trait Dao<T> {
    fn load(&self) -> Result<T>;
    fn save(&self, data: &T) -> Result<()>;
}
