use std::borrow::Cow;

use super::{character, PropertiesManager, StoreKey};

pub const STRINGS: &'static str = "/strings";

pub fn string(store: &PropertiesManager, key: &str) -> Option<Cow<'static, str>> {
    store
        .properties
        .get(&Into::<StoreKey>::into(STRINGS.to_owned() + key))
        .and_then(|s| s.clone().to_str())
}

pub fn register(store: &mut PropertiesManager) {
    store.add_prop(STRINGS.to_owned() + character::NAME, "Name");
    store.add_prop(STRINGS.to_owned() + character::GESCHLECHT, "Geschlecht");
}
