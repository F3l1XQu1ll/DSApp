use crate::spezies::SpeziesBase;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Achaz {
    /// modifiziere IN, sonst KO
    mod_in: bool,
    // modifiziere MU, sonst KK
    mod_mu: bool,
}

impl Achaz {
    pub fn new(mod_in: bool, mod_mu: bool) -> Self {
        Self { mod_in, mod_mu }
    }
}

impl SpeziesBase<'_> for Achaz {
    fn name() -> &'static str {
        "Achaz"
    }

    fn le() -> u8 {
        5
    }

    fn sk() -> i8 {
        -4
    }

    fn zk() -> i8 {
        -5
    }

    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
        if self.mod_in {
            eigenschaften.r#in += 1;
        } else {
            eigenschaften.ko += 1;
        }
        if self.mod_mu {
            eigenschaften.mu -= 1;
        } else {
            eigenschaften.kk -= 1;
        }
    }

    fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
        todo!()
    }

    fn cost() -> u8 {
        25
    }

    fn gs() -> u8 {
        8
    }
}
