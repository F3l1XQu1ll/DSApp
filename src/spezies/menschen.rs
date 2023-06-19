use super::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Menschen {
    mod_eigenschaft: crate::data::AttrType,
}

impl Menschen {
    pub fn new(mod_eigenschaft: crate::data::AttrType) -> Self {
        Self { mod_eigenschaft }
    }
}

impl SpeziesBase<'_> for Menschen {
    fn name() -> &'static str {
        "Menschen"
    }

    fn le() -> u8 {
        5
    }

    fn sk() -> i8 {
        -5
    }

    fn zk() -> i8 {
        -5
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        *eigenschaften.attr_mut(&self.mod_eigenschaft) += 1;
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        0
    }
}
