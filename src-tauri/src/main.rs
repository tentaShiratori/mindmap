// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    pub mod local_backend;
}

mod app_module;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
pub mod test {
    pub mod mock_file;
}
