
use crate::ui::*;
use slint::*;

pub fn setup(window: &AppWindow){
    window.global::<BackendRepository>().on_add(|backend_draft|{
        (false, slint::SharedString::from(backend_draft.name))
    });
}