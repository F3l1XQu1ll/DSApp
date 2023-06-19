use crate::data::{AttrType::*, CharakterTalentBase as Ctb, SteigerungsFaktor::*};

macro_rules! define_talent {
    ($name: expr, $stf: expr, $probe: expr) => {
        Ctb {
            name: $name,
            steigerungs_faktor: $stf,
            probe: $probe,
        }
    };
}

const FLIEGEN: Ctb = define_talent!("Fliegen", B, [MU, IN, GE]); // 0
const GAUKELEIEN: Ctb = define_talent!("Gaukelleien", A, [MU, CH, FF]); // 1
const KLETTERN: Ctb = define_talent!("Klettern", B, [MU, GE, KK]); // 2
const KOERPERBEHERRSCHUNG: Ctb = define_talent!("Körperbeherrschung", D, [GE, GE, KO]); // 3
const KRAFTAKT: Ctb = define_talent!("Kraftakt", B, [KO, KK, KK]); // 4
const REITEN: Ctb = define_talent!("Reiten", B, [CH, GE, KK]); // 5
const SCHWIMMEN: Ctb = define_talent!("Schwimmen", B, [GE, KO, KK]); // 6
const SELBSTBEHERRSCHUNG: Ctb = define_talent!("Selbstbeherrschung", D, [MU, MU, KO]); // 7
const SINGEN: Ctb = define_talent!("Singen", A, [KL, CH, KO]); // 8
const SINNESSCHAERFE: Ctb = define_talent!("Sinnesschärfe", D, [KL, IN, IN]); // 9
const TANZEN: Ctb = define_talent!("Tanzen", A, [KL, CH, GE]); // 10
const TASCHENDIEBSTAHL: Ctb = define_talent!("Taschendiebstahl", B, [MU, FF, GE]); // 11
const VERBERGEN: Ctb = define_talent!("Verbergen", C, [MU, IN, GE]); // 12
const ZECHEN: Ctb = define_talent!("Zechen", A, [KL, KO, KK]); // 13

const BEKEHREN: Ctb = define_talent!("Bekehren & Überzeugen", B, [MU, KL, CH]); // 14
const BETOEREN: Ctb = define_talent!("Betören", B, [MU, CH, CH]); // 15
const EINSCHUECHTERN: Ctb = define_talent!("Einschüchtern", B, [MU, IN, CH]); // 16
const ETIKETTE: Ctb = define_talent!("Etikette", B, [KL, IN, CH]); // 17
const GASSENWISSEN: Ctb = define_talent!("Gassenwissen", C, [KL, IN, CH]); // 18
const MENSCHENKENNTNIS: Ctb = define_talent!("Menschenkenntnis", C, [KL, IN, CH]); // 19
const UEBERREDEN: Ctb = define_talent!("Überreden", C, [MU, IN, CH]); // 20
const VERKLEIDEN: Ctb = define_talent!("Verkleiden", B, [IN, CH, GE]); // 21
const WILLENSKRAFT: Ctb = define_talent!("Willenskraft", D, [MU, IN, CH]); // 22

const FAERTENSUCHEN: Ctb = define_talent!("Fährtensuchen", C, [MU, IN, GE]); // 23
const FESSELN: Ctb = define_talent!("Fesseln", A, [KL, FF, KK]); // 24
const FISCHEN: Ctb = define_talent!("Fischen & Angeln", A, [FF, GE, KO]); // 25
const ORIENTIERUNG: Ctb = define_talent!("Orientierung", B, [KL, IN, IN]); // 26
const PFLANZENKUNDE: Ctb = define_talent!("Pflanzenkunde", C, [KL, FF, KO]); // 27
const TIERKUNDE: Ctb = define_talent!("Tierkunde", C, [MU, MU, CH]); // 28
const WILDNISLEBEN: Ctb = define_talent!("Wildnisleben", C, [MU, GE, KO]); // 29

const BRETTSPIEL: Ctb = define_talent!("Brett- & Glücksspiel", A, [KL, KL, IN]); // 30
const GEOGRAPHIE: Ctb = define_talent!("Geographie", B, [KL, KL, IN]); // 31
const GESCHICHTSWISSEN: Ctb = define_talent!("Geschichtswissen", B, [KL, KL, IN]); // 32
const KULTE: Ctb = define_talent!("Götter & Kulte", B, [KL, KL, IN]); // 33
const KRIEGSKUNST: Ctb = define_talent!("Kriegskunst", B, [MU, KL, IN]); // 34
const MAGIEKUNDE: Ctb = define_talent!("Magiekunde", C, [KL, KL, IN]); // 35
const MECHANIK: Ctb = define_talent!("Mechanik", B, [KL, KL, FF]); // 36
const RECHNEN: Ctb = define_talent!("Rechnen", A, [KL, KL, IN]); // 37
const RECHTSKUNDE: Ctb = define_talent!("Rechtskunde", A, [KL, KL, IN]); // 38
const SAGEN: Ctb = define_talent!("Sagen & Legenden", B, [KL, KL, IN]); // 39
const SPHAERENKUNDE: Ctb = define_talent!("Sphärenkunde", B, [KL, KL, IN]); // 40
const STERNKUNDE: Ctb = define_talent!("Sternkunde", A, [KL, KL, IN]); // 41

const ALCHIMIE: Ctb = define_talent!("Alchimie", C, [MU, KL, FF]); // 42
const SCHIFFE: Ctb = define_talent!("Boote & Schiffe", B, [FF, GE, KK]); // 43
const FAHRZEUGE: Ctb = define_talent!("Fahrzeuge", A, [CH, FF, KO]); // 44
const HANDEL: Ctb = define_talent!("Handel", B, [KL, IN, CH]); // 45
const HEILKUNDE_GIFT: Ctb = define_talent!("Heilkunde Gift", B, [MU, KL, IN]); // 46
const HEILKUNDE_KRANKHEITEN: Ctb = define_talent!("Heilkunde Krankheiten", B, [MU, IN, KO]); // 47
const HEILKUNDE_SEELE: Ctb = define_talent!("Heilkunde Seele", B, [IN, CH, KO]); // 48
const HEILKUNDE_WUNDEN: Ctb = define_talent!("Heilkunde Wunden", D, [KL, FF, FF]); // 49
const HOLZBEARBEITUNG: Ctb = define_talent!("Holzbearbeitung", B, [FF, GE, KK]); // 50
const LEBENSMITTELBEARBEITUNG: Ctb = define_talent!("Lebensmittelbearbeitung", A, [IN, FF, FF]); // 51
const LEDERBEARBEITUNG: Ctb = define_talent!("Lederbearbeitung", B, [FF, GE, KO]); // 52
const ZEICHNEN: Ctb = define_talent!("Malen & Zeichnen", A, [IN, FF, FF]); // 53
const METALLBEARBEITUNG: Ctb = define_talent!("Metallbearbeitung", C, [FF, KO, KK]); // 54
const MUSIZIEREN: Ctb = define_talent!("Musizieren", A, [CH, FF, KO]); // 55
const SCHLOESSERKNACKEN: Ctb = define_talent!("Schlösserknacken", C, [IN, FF, FF]); // 56
const STEINBEARBEITUNG: Ctb = define_talent!("Steinbearbeitung", A, [FF, FF, KK]); // 57
const STOFFBEARBEITUNG: Ctb = define_talent!("Stoffbearbeitung", A, [KL, FF, FF]); // 58

#[derive(
    Default, serde::Deserialize, serde::Serialize, Clone, enum_iterator::Sequence, PartialEq, Eq,
)]
pub enum CharakterTalentBases {
    // Köerpertalente
    #[default]
    FLIEGEN,
    GAUKELEIEN,
    KLETTERN,
    KOERPERBEHERRSCHUNG,
    KRAFTAKT,
    REITEN,
    SCHWIMMEN,
    SELBSTBEHERRSCHUNG,
    SINGEN,
    SINNESSCHAERFE,
    TANZEN,
    TASCHENDIEBSTAHL,
    VERBERGEN,
    ZECHEN,
    // Gesellschaftstalente
    BEKEHREN,
    BETOEREN,
    EINSCHUECHTERN,
    ETIKETTE,
    GASSENWISSEN,
    MENSCHENKENNTNIS,
    UEBERREDEN,
    VERKLEIDEN,
    WILLENSKRAFT,
    // Naturtalente
    FAERTENSUCHEN,
    FESSELN,
    FISCHEN,
    ORIENTIERUNG,
    PFLANZENKUNDE,
    TIERKUNDE,
    WILDNISLEBEN,
    // Wissenstalente
    BRETTSPIEL,
    GEOGRAPHIE,
    GESCHICHTSWISSEN,
    KULTE,
    KRIEGSKUNST,
    MAGIEKUNDE,
    MECHANIK,
    RECHNEN,
    RECHTSKUNDE,
    SAGEN,
    SPHAERENKUNDE,
    STERNKUNDE,
    // Handwerkstalente
    ALCHIMIE,
    SCHIFFE,
    FAHRZEUGE,
    HANDEL,
    HEILKUNDEGIFT,
    HEILKUNDEKRANKHEITEN,
    HEILKUNDESEELE,
    HEILKUNDEWUNDEN,
    HOLZBEARBEITUNG,
    LEBENSMITTELBEARBEITUNG,
    LEDERBEARBEITUNG,
    ZEICHNEN,
    METALLBEARBEITUNG,
    MUSIZIEREN,
    SCHLOESSERKNACKEN,
    STEINBEARBEITUNG,
    STOFFBEARBEITUNG,
}

impl CharakterTalentBases {
    pub fn to_talent_base(&self) -> Ctb {
        match *self {
            Self::FLIEGEN => FLIEGEN,
            Self::GAUKELEIEN => GAUKELEIEN,
            Self::KLETTERN => KLETTERN,
            Self::KOERPERBEHERRSCHUNG => KOERPERBEHERRSCHUNG,
            Self::KRAFTAKT => KRAFTAKT,
            Self::REITEN => REITEN,
            Self::SCHWIMMEN => SCHWIMMEN,
            Self::SELBSTBEHERRSCHUNG => SELBSTBEHERRSCHUNG,
            Self::SINGEN => SINGEN,
            Self::SINNESSCHAERFE => SINNESSCHAERFE,
            Self::TANZEN => TANZEN,
            Self::TASCHENDIEBSTAHL => TASCHENDIEBSTAHL,
            Self::VERBERGEN => VERBERGEN,
            Self::ZECHEN => ZECHEN,
            Self::BEKEHREN => BEKEHREN,
            Self::BETOEREN => BETOEREN,
            Self::EINSCHUECHTERN => EINSCHUECHTERN,
            Self::ETIKETTE => ETIKETTE,
            Self::GASSENWISSEN => GASSENWISSEN,
            Self::MENSCHENKENNTNIS => MENSCHENKENNTNIS,
            Self::UEBERREDEN => UEBERREDEN,
            Self::VERKLEIDEN => VERKLEIDEN,
            Self::WILLENSKRAFT => WILLENSKRAFT,
            Self::FAERTENSUCHEN => FAERTENSUCHEN,
            Self::FESSELN => FESSELN,
            Self::FISCHEN => FISCHEN,
            Self::ORIENTIERUNG => ORIENTIERUNG,
            Self::PFLANZENKUNDE => PFLANZENKUNDE,
            Self::TIERKUNDE => TIERKUNDE,
            Self::WILDNISLEBEN => WILDNISLEBEN,
            Self::BRETTSPIEL => BRETTSPIEL,
            Self::GEOGRAPHIE => GEOGRAPHIE,
            Self::GESCHICHTSWISSEN => GESCHICHTSWISSEN,
            Self::KULTE => KULTE,
            Self::KRIEGSKUNST => KRIEGSKUNST,
            Self::MAGIEKUNDE => MAGIEKUNDE,
            Self::MECHANIK => MECHANIK,
            Self::RECHNEN => RECHNEN,
            Self::RECHTSKUNDE => RECHTSKUNDE,
            Self::SAGEN => SAGEN,
            Self::SPHAERENKUNDE => SPHAERENKUNDE,
            Self::STERNKUNDE => STERNKUNDE,
            Self::ALCHIMIE => ALCHIMIE,
            Self::SCHIFFE => SCHIFFE,
            Self::FAHRZEUGE => FAHRZEUGE,
            Self::HANDEL => HANDEL,
            Self::HEILKUNDEGIFT => HEILKUNDE_GIFT,
            Self::HEILKUNDEKRANKHEITEN => HEILKUNDE_KRANKHEITEN,
            Self::HEILKUNDESEELE => HEILKUNDE_SEELE,
            Self::HEILKUNDEWUNDEN => HEILKUNDE_WUNDEN,
            Self::HOLZBEARBEITUNG => HOLZBEARBEITUNG,
            Self::LEBENSMITTELBEARBEITUNG => LEBENSMITTELBEARBEITUNG,
            Self::LEDERBEARBEITUNG => LEDERBEARBEITUNG,
            Self::ZEICHNEN => ZEICHNEN,
            Self::METALLBEARBEITUNG => METALLBEARBEITUNG,
            Self::MUSIZIEREN => MUSIZIEREN,
            Self::SCHLOESSERKNACKEN => SCHLOESSERKNACKEN,
            Self::STEINBEARBEITUNG => STEINBEARBEITUNG,
            Self::STOFFBEARBEITUNG => STOFFBEARBEITUNG,
        }
    }
}

pub const BODY_TALENTS: &'static [&'static Ctb] = &[
    &FLIEGEN,
    &GAUKELEIEN,
    &KLETTERN,
    &KOERPERBEHERRSCHUNG,
    &KRAFTAKT,
    &REITEN,
    &SCHWIMMEN,
    &SELBSTBEHERRSCHUNG,
    &SINGEN,
    &SINNESSCHAERFE,
    &TANZEN,
    &TASCHENDIEBSTAHL,
    &VERBERGEN,
    &ZECHEN,
];

pub const COMMUNITY_TALENTS: &[&Ctb] = &[
    &BEKEHREN,
    &BETOEREN,
    &EINSCHUECHTERN,
    &ETIKETTE,
    &GASSENWISSEN,
    &MENSCHENKENNTNIS,
    &UEBERREDEN,
    &VERKLEIDEN,
    &WILLENSKRAFT,
];

pub const NATURE_TALENTS: &[&Ctb; 7] = &[
    &FAERTENSUCHEN,
    &FESSELN,
    &FISCHEN,
    &ORIENTIERUNG,
    &PFLANZENKUNDE,
    &TIERKUNDE,
    &WILDNISLEBEN,
];

pub const KNOWLEDGE_TALENTS: &[&Ctb; 12] = &[
    &BRETTSPIEL,
    &GEOGRAPHIE,
    &GESCHICHTSWISSEN,
    &KULTE,
    &KRIEGSKUNST,
    &MAGIEKUNDE,
    &MECHANIK,
    &RECHNEN,
    &RECHTSKUNDE,
    &SAGEN,
    &SPHAERENKUNDE,
    &STERNKUNDE,
];

pub const HANDCRAFT_TALENTS: &[&Ctb; 17] = &[
    &ALCHIMIE,
    &SCHIFFE,
    &FAHRZEUGE,
    &HANDEL,
    &HEILKUNDE_GIFT,
    &HEILKUNDE_KRANKHEITEN,
    &HEILKUNDE_SEELE,
    &HEILKUNDE_WUNDEN,
    &HOLZBEARBEITUNG,
    &LEBENSMITTELBEARBEITUNG,
    &LEDERBEARBEITUNG,
    &ZEICHNEN,
    &METALLBEARBEITUNG,
    &MUSIZIEREN,
    &SCHLOESSERKNACKEN,
    &STEINBEARBEITUNG,
    &STOFFBEARBEITUNG,
];

#[derive(PartialEq, Eq, Clone, enum_iterator::Sequence)]
pub enum TalentClass {
    Body,
    Community,
    Nature,
    Knowledge,
    Handcraft,
}

impl TalentClass {
    pub fn description(&self) -> &'static str {
        match *self {
            TalentClass::Body => "Körpertalente",
            TalentClass::Community => "Gesellschaftstalente",
            TalentClass::Nature => "Naturtalente",
            TalentClass::Knowledge => "Wissenstalente",
            TalentClass::Handcraft => "Handwerkstalente",
        }
    }

    const fn talents(&self) -> &'static [&'static Ctb] {
        match *self {
            TalentClass::Body => BODY_TALENTS,
            TalentClass::Community => COMMUNITY_TALENTS,
            TalentClass::Nature => NATURE_TALENTS,
            TalentClass::Knowledge => KNOWLEDGE_TALENTS,
            TalentClass::Handcraft => HANDCRAFT_TALENTS,
        }
    }

    pub fn talents_iter(&self) -> std::slice::Iter<'_, &'static Ctb> {
        self.talents().iter()
    }

    const fn talent_index_offset(class: Self) -> usize {
        match class {
            TalentClass::Body => 0,
            TalentClass::Community => BODY_TALENTS.len(),
            TalentClass::Nature => {
                COMMUNITY_TALENTS.len() + Self::talent_index_offset(Self::Community)
            }
            TalentClass::Knowledge => {
                NATURE_TALENTS.len() + Self::talent_index_offset(Self::Nature)
            }
            TalentClass::Handcraft => {
                KNOWLEDGE_TALENTS.len() + Self::talent_index_offset(Self::Knowledge)
            }
        }
    }

    pub fn talent_index(&self, index: usize) -> usize {
        Self::talent_index_offset(self.clone()) + index
    }
}
