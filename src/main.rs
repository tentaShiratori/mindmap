#![windows_subsystem = "windows"]

mod ui {
    slint::include_modules!();
}
mod singletons {
    pub mod backend_repository;
    pub mod local_backend_repository;
    pub mod path_lib;
}

mod lib {
    pub mod dir;
    pub mod repository_json;
}
mod model {
    pub mod backend;
    pub mod local_backend;
}

use singletons::*;
use ui::*;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    backend_repository::setup(&ui);
    local_backend_repository::setup(&ui);
    path_lib::setup(&ui);
    ui.run()
}
