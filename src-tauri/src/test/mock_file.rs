use anyhow::Ok;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

pub struct MockFile {
    pub file: File,
    pub path: PathBuf,
}

impl Drop for MockFile {
    fn drop(&mut self) {
        fs::remove_file(self.path.clone()).unwrap();
    }
}

impl MockFile {
    fn random_file_name() -> String {
        rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }

    pub fn new() -> anyhow::Result<MockFile> {
        MockFile::_new(&(MockFile::random_file_name() + &String::from("json")))
    }

    fn _new(file_name: &str) -> anyhow::Result<MockFile> {
        let dir = env::temp_dir().join("mindmap-test");
        fs::create_dir_all(dir.clone())?;
        let file_path = dir.join(file_name);
        let tmp_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path.clone())?;
        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `tmp_dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        Ok(MockFile {
            file: tmp_file,
            path: file_path,
        })
    }
}
