use std::{borrow::Cow, collections::BTreeMap};

use egui::{Grid, Id, RichText};

use crate::{
    data::{AttrType, Kampftechnik, SteigerungsFaktor as StF},
    display::BuildUiNamed,
    drag_val,
    roll::{self, ShowRollEditor},
};

pub struct KampfTechnikenView<'a> {
    kampftechniken: &'a mut BTreeMap<Cow<'static, str>, Kampftechnik>,
}

impl<'a> KampfTechnikenView<'a> {
    pub fn new(kampftechniken: &'a mut BTreeMap<Cow<'static, str>, Kampftechnik>) -> Self {
        let default_skills = [
            Kampftechnik {
                id: "armbrueste".to_owned(),
                name: "Armbrüste".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::B,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "boegen".to_owned(),
                name: "Bögen".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "dolche".to_owned(),
                name: "Dolche".to_owned(),
                leiteigenschaft: AttrType::GE,
                steigerungs_faktor: StF::B,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "fechtwaffen".to_owned(),
                name: "Fechtwaffen".to_owned(),
                leiteigenschaft: AttrType::GE,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "hiebwaffen".to_owned(),
                name: "Hiebwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "kettenwaffen".to_owned(),
                name: "Kettenwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "lanzen".to_owned(),
                name: "Lanzen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::B,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "raufen".to_owned(),
                name: "Raufen".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: … oder KK, jeh nach dem was größer ist
                steigerungs_faktor: StF::B,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "schilde".to_owned(),
                name: "Schilde".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "schwerter".to_owned(),
                name: "Schwerter".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: oder KK
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "stangenwaffen".to_owned(),
                name: "Stangenwaffen".to_owned(),
                leiteigenschaft: AttrType::GE, // TODO: oder KK
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "wurfwaffen".to_owned(),
                name: "Wurfwaffen".to_owned(),
                leiteigenschaft: AttrType::FF,
                steigerungs_faktor: StF::B,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "zweihandhiebwaffen".to_owned(),
                name: "Zweihandhiebwaffen".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
            },
            Kampftechnik {
                id: "zweihandschwerter".to_owned(),
                name: "Zweihandschwerter".to_owned(),
                leiteigenschaft: AttrType::KK,
                steigerungs_faktor: StF::C,
                stufe: 6,
                ..Default::default()
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
                    ui.end_row();
                    ui.label(RichText::new(format!("mod: {}", skill.roll.mod_roll)));
                    ui.end_row();
                    skill.roll.ui_kampf_button(ui);
                });
            });
            ui.separator();
        }
    }

    pub fn ui_win(&mut self, ui: &mut egui::Ui) {
        for (name, skill) in &mut *self.kampftechniken {
            egui::Window::new("Kampfwurf")
                .open(&mut { skill.roll.show_editor == ShowRollEditor::Roll })
                .id(skill.id.clone().into())
                .show(ui.ctx(), |ui| {
                    ui.group(|ui| {
                        skill.roll.ui_kampf_mod(ui, skill.id.clone().into());
                        if ui.button("x").clicked() {
                            skill.roll.show_editor = ShowRollEditor::None;
                        }
                    });
                });
        }
    }
}
