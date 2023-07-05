use crate::{
    data::{Kampftechnik, Liturgy, Talent, Zauber},
    spezies::Spezies,
};

//Voraussetzungen allg implentiern
pub enum Voraussetzung {
    Spezies(Spezies),
    Kultur(),
    Vorteil(),
    Nachteil(),
}

//Vor-/Nachteile all implementieren
pub enum Vorteile {
    Vorteil,
    Nachteil,
}

pub struct Sonderfertigkeit {}

// Un-/Geeignete Vor-/nachteile einfügen
pub struct Profession {
    name: String,
    ap: u16,
    voraussetzung: Vec<Voraussetzung>,
    sonderfertigkeiten: Vec<Sonderfertigkeit>,
    kampftechniken: Vec<Kampftechnik>,
    talente: Vec<Talent>,
    zauber: Vec<Zauber>,
    liturgien: Vec<Liturgy>,
    e_vorteile: Vec<Vorteile>,
    e_nachteile: Vec<Vorteile>,
    u_vorteile: Vec<Vorteile>,
    u_nachteile: Vec<Vorteile>,
    varianten: Vec<Profession>,
}

#[macro_export]
macro_rules! profession {
    ($name_ident: ident, $name: expr, $ap: expr, $voraussetzung: expr, $sonderfertigkeiten: expr, $kampftechniken: expr, $talent: expr, $zauber: expr, $liturgien: expr, $e_vorteile: expr, $e_nachteile: expr, $u_vorteile: expr, $u_nachteile: expr, $varianten: expr) => {
        impl Profession {
            pub fn $name_ident() -> Profession {
                Profession {
                    name: $name,
                    ap: $ap,
                    voraussetzung: $voraussetzung,
                    sonderfertigkeiten: $sonderfertigkeiten,
                    kampftechniken: $kampftechniken,
                    talente: $talent,
                    zauber: $zauber,
                    liturgien: $liturgien,
                    e_vorteile: $e_vorteile,
                    e_nachteile: $e_nachteile,
                    u_vorteile: $u_vorteile,
                    u_nachteile: $u_nachteile,
                    varianten: $varianten,
                }
            }
        }
    };
}
