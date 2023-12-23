use crate::lib::repository_json::RepositoryJSON;
use crate::{
    lib::*,
    model::backend::*,
    ui,
    ui::{AppWindow, BackendDraft},
};
use anyhow::Result;
use slint::*;
use std::path;
use uuid::Uuid;

struct BackendRepository {
    backend_file_path: path::PathBuf,
}

impl repository_json::RepositoryJSON<Vec<Backend>> for BackendRepository {
    fn default_data(&self) -> Vec<Backend> {
        vec![]
    }
    fn json_path(&self) -> &path::Path {
        self.backend_file_path.as_path()
    }
}

impl BackendRepository {
    fn new() -> BackendRepository {
        let backend_file_path = dir::Dir::new().data().join("backend.json");
        BackendRepository {
            backend_file_path: backend_file_path,
        }
    }

    fn add(&self, draft: BackendDraft) -> Result<uuid::Uuid> {
        let id = Uuid::new_v4();
        let mut data = self.load()?;
        data.push(Backend {
            id: id.to_string(),
            name: draft.name.to_string(),
            r#type: draft.r#type,
        });
        self.save(&data)?;
        Ok(id)
    }
}

pub fn setup(window: &AppWindow) {
    window.global::<ui::BackendRepository>().on_add(|draft| {
        let result = BackendRepository::new().add(draft);

        match result {
            Ok(id) => (
                ui::Error {
                    did_occured: false,
                    message: "".into(),
                },
                id.to_string().into(),
            ),
            Err(err) => (
                ui::Error {
                    did_occured: true,
                    message: err.to_string().into(),
                },
                "".into(),
            ),
        }
    });
}
