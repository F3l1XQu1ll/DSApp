// ---------- Sprache ----------
pub enum Stufe {
    I,
    II,
    III,
    MS,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Sprache {
    pub name: String,
    //pub level: Stufe,
    pub cost: u8,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
// ---------- Schrift ---------
pub struct Schrift {
    pub name: String,
    pub cost: u8,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct SprachenSchriften {
    pub sprachen: Vec<Sprache>,
    pub schriften: Vec<Schrift>,
    pub show_editor: bool,
}
