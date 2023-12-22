
use crate::ui::*;
use slint::*;

pub fn setup(window: &AppWindow){
    window.global::<LocalBackendRepository>().on_edit(|local|{
        let path = local.path;
        true
    });
}