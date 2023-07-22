// ---------- Sprache ----------
#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum SpracheStufe {
    I,
    II,
    III,
    #[default]
    MS,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct Sprache {
    pub name: String,
    pub stufe: SpracheStufe,
    pub cost: u8,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
// ---------- Schrift ---------
pub struct Schrift {
    pub name: String,
    pub cost: u8,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct SprachenSchriften {
    pub sprachen: Vec<Sprache>,
    pub schriften: Vec<Schrift>,
    pub show_editor: bool,
}
