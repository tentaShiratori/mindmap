use crate::{app_module::AppModule, lib::dir::Dir, ui::*};
use slint::*;

pub fn setup(window: &AppWindow, _module: &'static AppModule) {
    window
        .global::<PathLib>()
        .set_home_dir(Dir::new().home().to_str().unwrap().into());
}
