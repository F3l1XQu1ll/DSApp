use super::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Orks {
    mod_kl: bool,
}

impl Orks {
    pub fn new(mod_kl: bool) -> Self {
        Self { mod_kl }
    }
}

impl SpeziesBase<'_> for Orks {
    fn name() -> &'static str {
        "Orks"
    }

    fn le() -> u8 {
        8
    }

    fn sk() -> i8 {
        -6
    }

    fn zk() -> i8 {
        -4
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        eigenschaften.mu += 1;
        eigenschaften.ko += 1;
        if self.mod_kl {
            eigenschaften.kl -= 1;
        } else {
            eigenschaften.ch -= 1;
        }
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        18
    }
}
