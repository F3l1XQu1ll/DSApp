use super::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Elfen {
    mod_in: bool,
    mod_kl: bool,
}

impl Elfen {
    pub fn new(mod_in: bool, mod_kl: bool) -> Self {
        Self { mod_in, mod_kl }
    }
}

impl SpeziesBase<'_> for Elfen {
    fn name() -> &'static str {
        "Elfen"
    }

    fn le() -> u8 {
        2
    }

    fn sk() -> i8 {
        -4
    }

    fn zk() -> i8 {
        -6
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        if self.mod_in {
            eigenschaften.r#in += 1;
        } else {
            eigenschaften.ge += 1;
        }

        if self.mod_kl {
            eigenschaften.kl -= 2;
        } else {
            eigenschaften.kk -= 2;
        }
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        18
    }
}
