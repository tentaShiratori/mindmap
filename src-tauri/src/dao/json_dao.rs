use crate::singletons::dao::Dao;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{prelude::*, SeekFrom};
use std::path::PathBuf;
use std::{fs, fs::File};

pub struct JsonDao<T> {
    pub path: PathBuf,
    pub default_data: T,
}

impl<T> JsonDao<T>
where
    T: Serialize,
    T: DeserializeOwned,
{
    fn open_file(&self) -> Result<File> {
        let path = self.path.as_path();
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
            let data = &serde_json::to_string(&self.default_data)?;
            writeln!(file, "{data}")?;
        }
        file.seek(SeekFrom::Start(0))?;
        Ok(file)
    }
}

impl<T> Dao<T> for JsonDao<T>
where
    T: Serialize,
    T: DeserializeOwned,
{
    fn load(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.open_file()?;
        let content = fs::read_to_string(self.path.clone())?;

        Ok(serde_json::from_str::<T>(&content)?)
    }

    fn save(&self, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let mut file = self.open_file()?;
        let data = &serde_json::to_string(data)?;
        writeln!(file, "{data}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::JsonDao;
    use crate::singletons::dao::Dao;
    use crate::test::mock_file::MockFile;
    use anyhow::Ok;
    use serde::{Deserialize, Serialize};
    use std::fs::{self};
    use std::io::Write;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Hoge {
        hoge: u32,
    }

    #[test]
    fn it_set_default_data_when_create_file() -> anyhow::Result<()> {
        let mock_file = MockFile::new()?;
        let json_dao = JsonDao {
            default_data: Hoge { hoge: 1 },
            path: mock_file.path.clone(),
        };
        json_dao.open_file()?;
        let contents = fs::read_to_string(mock_file.path.clone())?;
        assert_eq!(contents, String::from("{\"hoge\":1}\n"));

        Ok(())
    }

    #[test]
    fn it_load_data() -> anyhow::Result<()> {
        let mock_file = MockFile::new()?;
        writeln!(mock_file.file.try_clone()?, "{{\"hoge\":10}}")?;

        let json_dao = JsonDao {
            default_data: Hoge { hoge: 1 },
            path: mock_file.path.clone(),
        };

        let contents = json_dao.load()?;
        assert_eq!(contents, Hoge { hoge: 10 });

        Ok(())
    }

    #[test]
    fn it_save_data() -> anyhow::Result<()> {
        let mock_file = MockFile::new()?;

        let json_dao = JsonDao {
            default_data: Hoge { hoge: 1 },
            path: mock_file.path.clone(),
        };

        json_dao.save(&Hoge { hoge: 10 })?;

        assert_eq!(
            fs::read_to_string(mock_file.path.clone())?,
            String::from("{\"hoge\":10}\n")
        );

        Ok(())
    }
}
