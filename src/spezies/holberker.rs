use super::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Holberker;

impl Holberker {
    pub fn new() -> Self {
        Self
    }
}

impl SpeziesBase<'_> for Holberker {
    fn name() -> &'static str {
        "Holberker"
    }

    fn le() -> u8 {
        6
    }

    fn sk() -> i8 {
        -6
    }

    fn zk() -> i8 {
        -5
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        eigenschaften.kl -= 1;
        eigenschaften.ko += 1;
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        1
    }
}
