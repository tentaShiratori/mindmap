use crate::{lib::dir::Dir, ui::*};
use slint::*;

pub fn setup(window: &AppWindow) {
    window
        .global::<PathLib>()
        .set_home_dir(Dir::new().home().to_str().unwrap().into());
}
