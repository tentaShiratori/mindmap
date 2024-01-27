// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dao::{
    backend_json_dao::BackendJsonDaoParameters, json_dao::JsonDao,
    local_backend_json_dao::LocalBackendJsonDaoParameters,
};
use thiserror::Error;

use shaku::{module, HasComponent};
use tauri::{Manager, State};

use crate::{
    dao::{backend_json_dao::BackendJsonDao, local_backend_json_dao::LocalBackendJsonDao},
    singletons::add_local_backend_usecase::{
        AddLocalBackendArgs, AddLocalBackendUsecase, AddLocalBackendUsecaseImpl,
    },
};

module! {
    AppModule {
        components = [AddLocalBackendUsecaseImpl,LocalBackendJsonDao, BackendJsonDao],
        providers = []
    }
}
mod singletons {
    pub mod add_local_backend_usecase;
    pub mod dao;
}

mod dao {
    pub mod backend_json_dao;
    pub mod json_dao;
    pub mod local_backend_json_dao;
}
mod model {
    pub mod backend;
    pub mod backend_repository;
    pub mod local_backend;
    pub mod local_backend_repository;
}

use anyhow::Result;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn add_local_backend_usecase(name: &str, module: State<AppModule>) -> Result<String, String> {
    let usecase: &dyn AddLocalBackendUsecase = module.resolve_ref();

    usecase
        .exec(AddLocalBackendArgs {
            name: String::from("hoge"),
            path: String::from("huga"),
        })
        .or_else(|_| Err(String::from("Usecase Error")))
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("app data dir not found")]
    AppDataDirNotFound,
}
fn main() {
    let specta_builder = {
        // You can use `tauri_specta::js::builder` for exporting JS Doc instead of Typescript!`
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![add_local_backend_usecase]); // <- Each of your comments

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let specta_builder = specta_builder.path("../src/bindings.ts");

        specta_builder.into_plugin()
    };

    let builder = tauri::Builder::default();
    builder
        .setup(|app| {
            let handle = app.app_handle();
            let data_dir = handle
                .path_resolver()
                .app_data_dir()
                .ok_or(SetupError::AppDataDirNotFound)?;

            let module = AppModule::builder()
                .with_component_parameters::<LocalBackendJsonDao>(LocalBackendJsonDaoParameters {
                    json_dao: JsonDao {
                        path: data_dir.join("local_backend.json"),
                        default_data: vec![],
                    },
                })
                .with_component_parameters::<BackendJsonDao>(BackendJsonDaoParameters {
                    json_dao: JsonDao {
                        path: data_dir.join("backend.json"),
                        default_data: vec![],
                    },
                })
                .build();
            app.manage(module);
            Ok(())
        })
        .plugin(specta_builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
pub mod test {
    pub mod mock_file;
}
