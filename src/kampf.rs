use std::{borrow::Cow, collections::BTreeMap};

use egui::{Grid, Id, RichText};

use rand::prelude::*;

use crate::{
    data::{AttrType, Kampftechnik, SteigerungsFaktor as StF},
    display::BuildUiNamed,
    drag_val, drag_val_mod,
    roll::{self, Krit, ShowRollEditor},
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
                .open(&mut skill.roll.show_editor)
                .id(skill.id.clone().into())
                .show(ui.ctx(), |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            // modifier
                            ui.vertical(|ui| {
                                ui.label("Erschwernis / Erleichterung");
                                drag_val_mod!(ui, &mut skill.roll.mod_roll);
                            });
                            // KTW
                            ui.vertical(|ui| {
                                ui.label("KTW");
                                ui.code(skill.stufe.to_string());
                            });
                            // Leiteigenschaft
                            ui.vertical(|ui| {
                                ui.label("Leiteigenschaft");
                                ui.code(skill.leiteigenschaft.to_string());
                            });
                        });

                        //Wurf
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Würfeln").clicked() {
                                    skill.roll.rolld20[0] = rand::thread_rng().gen_range(1..21);
                                    skill.roll.rolld20[1] = rand::thread_rng().gen_range(1..21);
                                }
                                ui.code(skill.roll.rolld20[0].to_string());
                                if skill.roll.rolld20[0] == 1 || skill.roll.rolld20[0] == 20 {
                                    ui.code(skill.roll.rolld20[1].to_string());
                                }
                            });
                        });

                        //Würfel feld
                        ui.horizontal(|ui| {
                            // AT
                            ui.group(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("AT"));
                                    ui.end_row();
                                    // Wurf
                                    ui.vertical(|ui| {
                                        // Logik AT
                                        let roll_val: u8 =
                                            (skill.stufe as i8 + skill.roll.mod_roll) as u8;

                                        // Roll passed
                                        if skill.roll.rolld20[0] <= roll_val {
                                            skill.roll.passed = true;
                                        } else {
                                            skill.roll.passed = false
                                        }

                                        //Krit
                                        if skill.roll.rolld20[0] == 20 || skill.roll.rolld20[0] == 1
                                        {
                                            // Kritischer Erfolg
                                            if skill.roll.rolld20[0] == 1 {
                                                if skill.roll.rolld20[1] <= roll_val {
                                                    skill.roll.krit = Krit::KritischerErfolg
                                                } else {
                                                    skill.roll.krit = Krit::None;
                                                }
                                            }
                                            // Patzer
                                            if skill.roll.rolld20[0] == 20 {
                                                skill.roll.passed = false;
                                                if skill.roll.rolld20[1] > roll_val {
                                                    skill.roll.krit = Krit::Patzer;
                                                } else {
                                                    skill.roll.krit = Krit::None
                                                }
                                            }
                                        } else {
                                            // kein Krit

                                            skill.roll.krit = Krit::None
                                        }

                                        if skill.roll.passed {
                                            ui.label("wurf bestanden!");
                                        } else {
                                            ui.label("wurf fehlgeschlagen!");
                                        }
                                        if skill.roll.krit == Krit::Patzer {
                                            ui.label("Patzer !!!");
                                        } else {
                                            if skill.roll.krit == Krit::KritischerErfolg {
                                                ui.label("Kritischer Erfolg !!!");
                                            }
                                        }
                                    });
                                });
                            });
                            // PA
                            ui.group(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("PA"));
                                    ui.end_row();
                                    // Wurf
                                    ui.vertical(|ui| {
                                        // Logik AT
                                        let roll_val: u8 = (skill.stufe as i8 / 2
                                            + (skill.stufe as i8 % 2)
                                            + skill.roll.mod_roll)
                                            as u8;
                                        // Roll passed
                                        if skill.roll.rolld20[0] <= roll_val {
                                            skill.roll.passed = true;
                                        } else {
                                            skill.roll.passed = false
                                        }

                                        //Krit
                                        if skill.roll.rolld20[0] == 20 || skill.roll.rolld20[0] == 1
                                        {
                                            // Kritischer Erfolg
                                            if skill.roll.rolld20[0] == 1 {
                                                if skill.roll.rolld20[1] <= roll_val {
                                                    skill.roll.krit = Krit::KritischerErfolg
                                                } else {
                                                    skill.roll.krit = Krit::None;
                                                }
                                            }
                                            // Patzer
                                            if skill.roll.rolld20[0] == 20 {
                                                skill.roll.passed = false;
                                                if skill.roll.rolld20[1] > roll_val {
                                                    skill.roll.krit = Krit::Patzer;
                                                } else {
                                                    skill.roll.krit = Krit::None
                                                }
                                            }
                                        } else {
                                            // kein Krit
                                            skill.roll.krit = Krit::None
                                        }

                                        if skill.roll.passed {
                                            ui.label("wurf bestanden!");
                                        } else {
                                            ui.label("wurf fehlgeschlagen!");
                                        }
                                        if skill.roll.krit == Krit::Patzer {
                                            ui.label("Patzer !!!");
                                        } else {
                                            if skill.roll.krit == Krit::KritischerErfolg {
                                                ui.label("Kritischer Erfolg !!!");
                                            }
                                        }
                                    });
                                });
                            });
                        });
                        //if ui.button("schließen").clicked() {
                        //    skill.roll.show_editor = ShowRollEditor::None;
                        //}
                    });
                });
        }
    }
}
