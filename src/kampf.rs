use std::{borrow::Cow, collections::BTreeMap};

use egui::{Grid, RichText};

use crate::{
    data::{AttrType, Kampftechnik, SteigerungsFaktor as StF},
    display::BuildUiNamed,
    drag_val,
};

pub struct KampfTechnikenView<'a> {
    kampftechniken: &'a mut BTreeMap<Cow<'static, str>, Kampftechnik>,
}

impl<'a> KampfTechnikenView<'a> {
    pub fn new(kampftechniken: &'a mut BTreeMap<Cow<'static, str>, Kampftechnik>) -> Self {
        let default_skills = [
            Kampftechnik {
                name: "Armbrüste".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::B,
                stufe: 6,
            },
            Kampftechnik {
                name: "Bögen".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Dolche".to_owned(),
                leiteigenschaft: AttrType::GE,
                steigerungs_faktor: StF::B,
                stufe: 6,
            },
            Kampftechnik {
                name: "Fechtwaffen".to_owned(),
                leiteigenschaft: AttrType::GE,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Hiebwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Kettenwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Lanzen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::B,
                stufe: 6,
            },
            Kampftechnik {
                name: "Raufen".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: … oder KK, jeh nach dem was größer ist
                steigerungs_faktor: StF::B,
                stufe: 6,
            },
            Kampftechnik {
                name: "Schilde".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Schwerter".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: oder KK
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Stangenwaffen".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: oder KK
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Wurfwaffen".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::B,
                stufe: 6,
            },
            Kampftechnik {
                name: "Zweihandhiebwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
            Kampftechnik {
                name: "Zweihandschwerter".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
            },
        ];

        for skill in default_skills {
            kampftechniken
                .entry(skill.name.clone().into())
                .or_insert(skill);
        }
        Self { kampftechniken }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        for (name, skill) in &mut *self.kampftechniken {
            ui.collapsing(RichText::new(name.to_owned()).heading(), |ui| {
                Grid::new(format!("kampftechnik-{}", name)).show(ui, |ui| {
                    ui.label("Leiteigenschaft");
                    skill
                        .leiteigenschaft
                        .ui(ui, format!("le-select-{}", name).as_str());
                    ui.end_row();
                    skill
                        .steigerungs_faktor
                        .ui(ui, format!("stf-select-{}", name).as_str());
                    ui.end_row();
                    drag_val!(ui, "Level", &mut skill.stufe, min: 6);
                    ui.end_row();
                    ui.label("Kosten (AP)");
                    ui.label(RichText::new(format!("{}", skill.cost())));
                });
            });
            ui.separator();
        }
    }
}
