use std::borrow::Cow;

use egui::Ui;

pub mod character;

use crate::properties::{strings::string, PropertiesManager, Value};

pub trait View {
    fn view(store: &mut PropertiesManager, ui: &mut Ui);
}

#[allow(unused)]
pub fn text_edit(
    ui: &mut Ui,
    store: &PropertiesManager,
    key: Cow<'static, str>,
    handle: &mut Value,
) {
    ui.label(string(store, &key).expect("Property should have associated string!"));
    ui.text_edit_singleline(handle.str_mut());
}
