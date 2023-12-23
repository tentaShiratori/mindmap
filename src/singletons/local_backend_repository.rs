use crate::lib::repository_json::RepositoryJSON;
use crate::{
    lib::*,
    model::local_backend::*,
    ui,
    ui::{AppWindow, Local},
};
use anyhow::Result;
use slint::*;
use std::path;

struct LocalBackendRepository {
    local_backend_file_path: path::PathBuf,
}

impl repository_json::RepositoryJSON<Vec<LocalBackend>> for LocalBackendRepository {
    fn default_data(&self) -> Vec<LocalBackend> {
        vec![]
    }
    fn json_path(&self) -> &path::Path {
        self.local_backend_file_path.as_path()
    }
}

impl LocalBackendRepository {
    fn new() -> LocalBackendRepository {
        let local_backend_file_path = dir::Dir::new().data().join("local_backend.json");
        LocalBackendRepository {
            local_backend_file_path,
        }
    }

    fn edit(&self, local: Local) -> Result<()> {
        let mut data = self.load()?;
        data.push(LocalBackend {
            backend_id: local.backend_id.to_string(),
            path: local.path.to_string(),
        });
        self.save(&data)?;
        Ok(())
    }
}

pub fn setup(window: &AppWindow) {
    window
        .global::<ui::LocalBackendRepository>()
        .on_edit(|local| {
            let local_backend_repository = LocalBackendRepository::new();
            let result = local_backend_repository.edit(local);

            match result {
                Ok(_) => ui::Error {
                    did_occured: false,
                    message: "".into(),
                },
                Err(err) => ui::Error {
                    did_occured: true,
                    message: err.to_string().into(),
                },
            }
        });
}
