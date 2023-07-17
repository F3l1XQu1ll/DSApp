use egui::Ui;

use super::View;

pub struct Character;

impl View for Character {
    fn view(_store: &mut crate::properties::PropertiesManager, _ui: &mut Ui) {
        // FIXME: does not compile yet
        /*let character_properties = store
            .children_mut(properties::character::PERSONAL)
            .collect::<Vec<_>>();
        for (k, v) in character_properties {
            super::text_edit(ui, store, std::borrow::Cow::Borrowed(k), v);
        }*/
    }
}
