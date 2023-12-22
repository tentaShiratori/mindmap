pub mod ui {
    slint::include_modules!();
}
mod singletons{
    pub mod backend_repository;
    pub mod local_backend_repository;
}
use ui::*;
use singletons::*;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    backend_repository::setup(&ui);
    local_backend_repository::setup(&ui);

    ui.run()
}
