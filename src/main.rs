#![cfg_attr(not(test), windows_subsystem = "windows")]

mod extends {
    pub mod ui;
}

mod ui {
    slint::include_modules!();
}
mod singletons {
    pub mod backend_repository;
    pub mod dao;
    pub mod local_backend_repository;
    pub mod path_lib;
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

use app_module::AppModule;
use singletons::*;
use ui::*;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let module: &'static AppModule = Box::leak(Box::new(AppModule::new()));

    backend_repository::setup(&ui, module);
    local_backend_repository::setup(&ui, module);
    path_lib::setup(&ui, module);
    ui.run()
}

#[cfg(test)]
pub mod test {
    pub mod mock_file;
}
