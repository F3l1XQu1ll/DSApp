use super::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Zwerge {
    mod_ch: bool,
}

impl Zwerge {
    pub fn new(mod_ch: bool) -> Self {
        Self { mod_ch }
    }
}

impl SpeziesBase<'_> for Zwerge {
    fn name() -> &'static str {
        "Zwerge"
    }

    fn le() -> u8 {
        8
    }

    fn sk() -> i8 {
        -4
    }

    fn zk() -> i8 {
        -4
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        eigenschaften.ko += 1;
        eigenschaften.kk += 1;
        if self.mod_ch {
            eigenschaften.ch -= 2;
        } else {
            eigenschaften.ge -= 2;
        }
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        61
    }

    fn gs() -> u8 {
        6
    }
}
