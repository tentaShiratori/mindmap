use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{prelude::*, SeekFrom};
use std::{fs, fs::File, path::Path};

pub trait RepositoryJSON<T>
where
    T: Serialize,
    T: DeserializeOwned,
{
    fn json_path(&self) -> &Path;
    fn default_data(&self) -> T;
    fn open_file(&self) -> Result<File> {
        let path = self.json_path();
        let parent = path.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let metadata = file.metadata()?;
        if metadata.len() == 0 {
            file.write_all(&serde_json::to_string(&self.default_data())?.into_bytes())?;
        }
        file.seek(SeekFrom::Start(0))?;
        Ok(file)
    }

    fn load(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut file = self.open_file()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(serde_json::from_str::<T>(buf.as_str())?)
    }

    fn save(&self, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let mut file = self.open_file()?;
        file.write_all(&serde_json::to_string(data)?.into_bytes())?;

        Ok(())
    }
}
