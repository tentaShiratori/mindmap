// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path;

use shaku::{module, HasComponent};
use tauri::State;

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

mod lib {
    pub mod dir;
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

fn main() {
    let module = AppModule::builder().build();

    let specta_builder = {
        // You can use `tauri_specta::js::builder` for exporting JS Doc instead of Typescript!`
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![add_local_backend_usecase]); // <- Each of your comments

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let specta_builder = specta_builder.path("../src/bindings.ts");

        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .manage(module)
        .plugin(specta_builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
pub mod test {
    pub mod mock_file;
}
