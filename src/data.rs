use crate::{character_talents::CharakterTalentBases, erfahrungsgrade::Erfahrungsgrade};

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct KaP {
    pub max: u16,
    pub current: u16,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Attributes {
    pub mu: u8,
    pub kl: u8,
    pub r#in: u8,
    pub ch: u8,
    pub ff: u8,
    pub ge: u8,
    pub ko: u8,
    pub kk: u8,
}

impl Attributes {
    pub fn attr_mut(&mut self, attr: &AttrType) -> &mut u8 {
        match &attr {
            AttrType::Any => panic!("Any darf nicht gewählt werden!"),
            AttrType::MU => &mut self.mu,
            AttrType::KL => &mut self.kl,
            AttrType::IN => &mut self.r#in,
            AttrType::CH => &mut self.ch,
            AttrType::FF => &mut self.ff,
            AttrType::GE => &mut self.ge,
            AttrType::KO => &mut self.ko,
            AttrType::KK => &mut self.kk,
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
pub enum CostType {
    #[default]
    AsP,
}

impl ToString for CostType {
    fn to_string(&self) -> String {
        match *self {
            CostType::AsP => "AsP",
        }
        .to_string()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Cost {
    pub amount: u32,
    pub costs_type: CostType,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Probe(pub [AttrType; 3]);

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
pub enum Range {
    #[default]
    CastOnSelf,
    Touch,
    Foot(u32),
}

impl ToString for Range {
    fn to_string(&self) -> String {
        match *self {
            Range::CastOnSelf => "Selbst".to_string(),
            Range::Touch => "Berührung".to_string(),
            Range::Foot(f) => format!("{f} Fuß"),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
pub enum SteigerungsFaktor {
    #[default]
    A,
    B,
    C,
    D,
}

impl ToString for SteigerungsFaktor {
    fn to_string(&self) -> String {
        match *self {
            SteigerungsFaktor::A => "A",
            SteigerungsFaktor::B => "B",
            SteigerungsFaktor::C => "C",
            SteigerungsFaktor::D => "D",
        }
        .to_string()
    }
}

impl SteigerungsFaktor {
    pub fn cost(&self, rank: u16, profession: bool) -> u32 {
        let scale = self.scale();
        let border = if scale < 5 { 12 } else { 14 }; // E scales a little different, not implemented
        if rank <= border {
            if profession {
                (rank * scale).into()
            } else {
                (scale + rank * scale).into()
            }
        } else {
            // up to level 12
            let base_cost = 12 * scale;
            // add accumulating amount for each successive level
            let start = if !profession { base_cost as u32 } else { 0 };
            start
                + (1..(rank - 11))
                    .map(|i| i as u32 * scale as u32)
                    .fold(0, |acc, i| acc + i)
        }
    }

    pub fn scale(&self) -> u16 {
        match *self {
            SteigerungsFaktor::A => 1,
            SteigerungsFaktor::B => 2,
            SteigerungsFaktor::C => 3,
            SteigerungsFaktor::D => 4,
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Liturgy {
    pub name: String,
    pub probe: Probe,
    pub fw: u16,
    pub cost: Cost,
    pub duration: u32,
    pub effect_duration: u32,
    pub range: Range,
    pub aspect: String,
    pub stf: SteigerungsFaktor,
    pub effect_name: String,
    pub effect: String,
    pub s: u32,
    pub show_editor: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Talent {
    pub name: String,
    pub description: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
pub enum Archetype {
    #[default]
    Cleric,
    Mage,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Equipment {
    pub name: String,
    pub weight: u16,
    pub location: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Animal {
    pub name: String,
    pub typus: String,
    pub lo: u16,
    pub ap: u16,
    pub lep: u16,
    pub asp: u16,
    pub mu: u16,
    pub kl: u16,
    pub r#in: u16,
    pub ch: u16,
    pub ff: u16,
    pub ge: u16,
    pub ko: u16,
    pub kk: u16,
    pub sk: u16,
    pub zk: u16,
    pub rs: u16,
    pub ini: u16,
    pub gs: u16,
    pub attack: u16,
    pub at: u16,
    pub vw: u16,
    pub tp: u16,
    pub rw: u16,
    pub actions: Vec<String>,
    pub abilities: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
    Diverse(String),
}

impl Default for Gender {
    fn default() -> Self {
        Self::Diverse(String::new())
    }
}
#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum AttrType {
    #[default]
    Any,
    MU,
    KL,
    IN,
    CH,
    FF,
    GE,
    KO,
    KK,
}

impl std::ops::Add<i32> for AttrType {
    type Output = AttrChanges;

    fn add(self, rhs: i32) -> Self::Output {
        AttrChanges::Plus(self, rhs)
    }
}

impl std::ops::Sub<i32> for AttrType {
    type Output = AttrChanges;

    fn sub(self, rhs: i32) -> Self::Output {
        AttrChanges::Plus(self, rhs * -1)
    }
}

impl ToString for AttrType {
    fn to_string(&self) -> String {
        match *self {
            AttrType::Any => "Beliebig",
            AttrType::MU => "MU",
            AttrType::KL => "KL",
            AttrType::IN => "IN",
            AttrType::CH => "CH",
            AttrType::FF => "FF",
            AttrType::GE => "GE",
            AttrType::KO => "KO",
            AttrType::KK => "KK",
        }
        .to_string()
    }
}

impl AttrType {
    pub fn to_string_long(&self) -> String {
        match *self {
            AttrType::Any => "Beliebig",
            AttrType::MU => "Mut",
            AttrType::KL => "Klugheit",
            AttrType::IN => "Intuition",
            AttrType::CH => "Charisma",
            AttrType::FF => "Fingerfertigkeit",
            AttrType::GE => "Gewandheit",
            AttrType::KO => "Konstitution",
            AttrType::KK => "Körperkraft",
        }
        .to_string()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum AttrChanges {
    #[default]
    None,
    Plus(AttrType, i32),
    And(Box<AttrChanges>, Box<AttrChanges>),
    Or(Box<AttrChanges>, Box<AttrChanges>),
}

impl std::ops::BitAnd for AttrChanges {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::BitOr for AttrChanges {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Box::new(self), Box::new(rhs))
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct SpeciesBaseAttributes {
    pub ap: i16,
    pub le: i16,
    pub sk: i16,
    pub zk: i16,
    pub gs: i16,
    pub initial_attribute_change: AttrChanges,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Species {
    Achaz,
    Elfen,
    Halbelfen,
    Halborks,
    Holberker,
    #[default]
    Menschen,
    Orks,
    Zwerge,
}

impl Species {
    pub fn attributes(&self) -> SpeciesBaseAttributes {
        match *self {
            Species::Achaz => SpeciesBaseAttributes {
                ap: 25,
                le: 5,
                sk: -4,
                zk: -5,
                gs: 8,
                initial_attribute_change: (AttrType::IN + 1 & AttrType::KO + 1)
                    & (AttrType::MU - 1 | AttrType::KK - 1),
            },
            Species::Elfen => SpeciesBaseAttributes {
                ap: 18,
                le: 2,
                sk: -4,
                zk: -6,
                gs: 8,
                initial_attribute_change: (AttrType::IN + 1 & AttrType::GE + 1)
                    & (AttrType::KL - 2 | AttrType::KK - 2),
            },
            Species::Halbelfen => SpeciesBaseAttributes {
                ap: 0,
                le: 5,
                sk: -4,
                zk: -6,
                gs: 8,
                initial_attribute_change: AttrType::Any + 1,
            },
            Species::Halborks => SpeciesBaseAttributes {
                ap: 1,
                le: 6,
                sk: -6,
                zk: -5,
                gs: 8,
                initial_attribute_change: AttrType::KO + 1 & AttrType::CH - 1,
            },
            Species::Holberker => SpeciesBaseAttributes {
                ap: 1,
                le: 6,
                sk: -6,
                zk: -5,
                gs: 8,
                initial_attribute_change: AttrType::KL - 1 & AttrType::KO + 1,
            },
            Species::Menschen => SpeciesBaseAttributes {
                ap: 0,
                le: 5,
                sk: -5,
                zk: -5,
                gs: 8,
                initial_attribute_change: AttrType::Any + 1,
            },
            Species::Orks => SpeciesBaseAttributes {
                ap: 18,
                le: 8,
                sk: -6,
                zk: -4,
                gs: 8,
                initial_attribute_change: (AttrType::MU + 1 & AttrType::KO + 1)
                    & (AttrType::KL - 1 | AttrType::CH - 1),
            },
            Species::Zwerge => SpeciesBaseAttributes {
                ap: 61,
                le: 8,
                sk: -4,
                zk: -4,
                gs: 6,
                initial_attribute_change: (AttrType::KO + 1 & AttrType::KK + 1)
                    & (AttrType::CH - 2 | AttrType::GE - 2),
            },
        }
    }
}

impl ToString for Species {
    fn to_string(&self) -> String {
        match self {
            Species::Achaz => "Achaz",
            Species::Elfen => "Elf",
            Species::Halbelfen => "Halbelf",
            Species::Halborks => "Halbork",
            Species::Holberker => "Holberker",
            Species::Menschen => "Mensch",
            Species::Orks => "Ork",
            Species::Zwerge => "Zwerg",
        }
        .to_string()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct AttrAPCost {
    pub ap_cost: u32,
    pub name: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Kampftechnik {
    pub name: String,
    pub leiteigenschaft: AttrType,
    pub steigerungs_faktor: SteigerungsFaktor,
    pub stufe: u16,
}

impl Kampftechnik {
    pub fn cost(&self) -> u16 {
        self.steigerungs_faktor.scale() * (self.stufe - 6) // Kosten bis 12 sind Stufe * Faktor
            + if self.stufe >= 12 { // Ab Stufe 13 ...
                (1..(self.stufe - 11))  // zuzüglich des Faktors 
                    .map(|i| i * self.steigerungs_faktor.scale()) // multipliziert mit jeder extra Stufe
                    .fold(0, |acc, i| acc + i) // welcher pro extra Stufe aufsummiert wird
            } else {
                0
            }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct CharakterTalent {
    pub base: CharakterTalentBases,
    pub stufe: u16,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct CharakterTalentBase {
    pub name: &'static str,
    pub steigerungs_faktor: SteigerungsFaktor,
    pub probe: [AttrType; 3],
}

impl CharakterTalentBase {
    pub fn probe(&self) -> &[AttrType; 3] {
        &self.probe
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Profession {
    pub name: String,
    pub ap_cost: u32,
    pub preconditions: Vec<AttrAPCost>,
    pub specials: Vec<AttrAPCost>,
    pub fighting: Vec<Kampftechnik>,
    pub talents: Vec<CharakterTalent>,
    pub zaubertrick: Talent,
    // pub zauber: Vec<>
    pub show_editor: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Identity {
    pub name: String,
    pub gender: Gender,
    // pub species: Species,
    pub species: crate::spezies::Spezies,
    pub culture: String,
    pub social_rank: String,
    pub hometown: String,
    pub profession: Profession,
    pub family: String,
    pub age_date_of_birth: String,
    pub hair_color: String,
    pub eye_color: String,
    pub size: String,
    pub weight: String,
    pub characteristical: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Character {
    pub identity: Identity,
    pub spells: ZauberTable,
    pub talents: std::collections::BTreeMap<usize, u16>,
    pub erfahrungsgrad: Erfahrungsgrade,
    pub attributes: Attributes,
    pub kampftechniken: std::collections::BTreeMap<std::borrow::Cow<'static, str>, Kampftechnik>,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Zielkategorie {
    Zone(Option<&'static str>),
    Objekte(Option<&'static str>),
    ProfaneObjekte,
    Wesen,
    ObjekteWesen,
    Kulturschaffende,
    KulturschaffendeSelbst,
    KulturschaffendeUebernatuerlicheWesen,
    KulturschaffendeVerstorbene,
    Lebewesen(Option<&'static str>),
    Pflanze,
    BeseelteGeister,
    Tiere(Option<&'static str>),
    Elementare,
    Daemonen,
    Feenwesen,
    Zauber,
    #[default]
    Alle,
}

impl ToString for Zielkategorie {
    fn to_string(&self) -> String {
        match *self {
            Zielkategorie::Zone(Some(z)) => format!("Zone ({z})"),
            Zielkategorie::Zone(None) => "Zone".to_string(),
            Zielkategorie::Objekte(Some(o)) => format!("Objekte ({o})"),
            Zielkategorie::Objekte(None) => "Objekte".to_string(),
            Zielkategorie::ProfaneObjekte => "Profane Objekte".to_string(),
            Zielkategorie::Wesen => "Wesen".to_string(),
            Zielkategorie::ObjekteWesen => "Objekte, Wesen".to_string(),
            Zielkategorie::Kulturschaffende => "Kulturschaffende".to_string(),
            Zielkategorie::KulturschaffendeSelbst => "Kulturschaffende (Selbst)".to_string(),
            Zielkategorie::KulturschaffendeUebernatuerlicheWesen => {
                "Kulturschaffende (Übernatürliche Wesen)".to_string()
            }
            Zielkategorie::KulturschaffendeVerstorbene => {
                "Kulturschaffende (Verstorbene)".to_string()
            }
            Zielkategorie::Lebewesen(Some(l)) => format!("Lebewesen ({l})"),
            Zielkategorie::Lebewesen(None) => "Lebewesen".to_string(),
            Zielkategorie::Pflanze => "Pflanze".to_string(),
            Zielkategorie::BeseelteGeister => "Beseelte Geister".to_string(),
            Zielkategorie::Tiere(Some(t)) => format!("Tiere ({t})"),
            Zielkategorie::Tiere(None) => "Tiere".to_string(),
            Zielkategorie::Elementare => "Elementare".to_string(),
            Zielkategorie::Daemonen => "Dämonen".to_string(),
            Zielkategorie::Feenwesen => "Feenwesen".to_string(),
            Zielkategorie::Zauber => "Zauber".to_string(),
            Zielkategorie::Alle => "Alle".to_string(),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Merkmal {
    #[default]
    Antimagie,
    Objekt,
    Heilung,
    Verwandlung,
    Elementar,
    Daemonisch,
    Einfluss,
    Hellsicht,
    Illusion,
    Sphaeren,
    Telekinese,
    Temporal,
}

impl ToString for Merkmal {
    fn to_string(&self) -> String {
        match *self {
            Merkmal::Antimagie => "Antimagie",
            Merkmal::Objekt => "Objekt",
            Merkmal::Heilung => "Heilung",
            Merkmal::Verwandlung => "Verwandlung",
            Merkmal::Elementar => "Elementar",
            Merkmal::Daemonisch => "Dämonisch",
            Merkmal::Einfluss => "Einfluss",
            Merkmal::Hellsicht => "Hellsicht",
            Merkmal::Illusion => "Illusion",
            Merkmal::Sphaeren => "Sphären",
            Merkmal::Telekinese => "Telekinese",
            Merkmal::Temporal => "Temporal",
        }
        .to_string()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum Verbreitung {
    #[default]
    Allgemein,
    Gildenmagier,
    Druiden,
    Elfen,
    Goblinzauberinnen,
    Kristallomanten,
    Hexen,
    Scharlatane,
    Geoden,
    Blutkrieger,
    Nachtalben,
    Borbaradianer,
}

impl ToString for Verbreitung {
    fn to_string(&self) -> String {
        match *self {
            Verbreitung::Allgemein => "Allgemein",
            Verbreitung::Gildenmagier => "Gildenmagier",
            Verbreitung::Druiden => "Druiden",
            Verbreitung::Elfen => "Elfen",
            Verbreitung::Goblinzauberinnen => "Goblinzauberinnen",
            Verbreitung::Kristallomanten => "Kristallomanten",
            Verbreitung::Hexen => "Hexen",
            Verbreitung::Scharlatane => "Scharlatane",
            Verbreitung::Geoden => "Geoden",
            Verbreitung::Blutkrieger => "Blutkrieger",
            Verbreitung::Nachtalben => "Nachtalben",
            Verbreitung::Borbaradianer => "Borbaradianer",
        }
        .to_string()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Erweiterung {
    pub name: &'static str,
    pub fw: u16,
    pub ap: u16,
    pub desc: &'static str,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum ProbeModifikator {
    #[default]
    ZK,
    SK,
}

impl ToString for ProbeModifikator {
    fn to_string(&self) -> String {
        match *self {
            ProbeModifikator::ZK => "ZK",
            ProbeModifikator::SK => "SK",
        }
        .to_string()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum SpecialAsP {
    N(u16),
    // bei tier
    KL,
    // min wert
    Min(&'static str),
}

impl Default for SpecialAsP {
    fn default() -> Self {
        Self::N(0)
    }
}

impl ToString for SpecialAsP {
    fn to_string(&self) -> String {
        match *self {
            SpecialAsP::N(n) => n.to_string(),
            SpecialAsP::KL => "KL".to_string(),
            SpecialAsP::Min(m) => m.to_string(),
        }
    }
}

// Zauber sind const und werden nur referenziert, müssen also nicht geclont oder serialisiert werden
pub struct Zauber {
    pub name: &'static str,
    pub probe: [AttrType; 3],
    pub probe_modifikator: Option<ProbeModifikator>,
    pub wirkung: &'static str,
    pub dauer: u16,
    pub dauer_modifiable: bool,
    pub asp: SpecialAsP,
    pub asp_modifiable: bool,
    pub reichweite: Range,
    pub reichweite_modifiable: bool,
    pub wirkungsdauer: &'static str,
    pub zielkategorie: Zielkategorie,
    pub merkmal: Merkmal,
    pub verbreitung: &'static [Verbreitung],
    pub stf: SteigerungsFaktor,
    pub erweiterungen: &'static [Erweiterung],
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct ZauberDescriptor {
    pub level: u8,
    pub enabled_extensions: std::collections::BTreeSet<usize>,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct ZauberTable {
    pub enabled_spells: std::collections::BTreeMap<usize, ZauberDescriptor>,
    pub search: String,
    pub show_selector: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Erfahrungsgrad {
    pub name: &'static str,
    pub ap_konto: u16,
    pub eigenschaft_max: u8,
    pub fertigkeit_max: u8,
    pub kampftechnik_max: u8,
    pub eingenschaftspunkte_max: u8,
    pub zauber_max: u8,
    pub fremdzauber: u8,
}
