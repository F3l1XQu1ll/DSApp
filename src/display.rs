use log::Level;
use tracing::debug;

use crate::data::{
    AttrAPCost, AttrType, Character, CharakterTalent, Gender, Kampftechnik, Profession, Species,
    SteigerungsFaktor, ZauberDescriptor, ZauberTable,
};

use crate::sprachen_schriften::{Schrift, Sprache, SpracheStufe, SprachenSchriften};

use crate::spezies::prelude::*;

use crate::character_talents::CharakterTalentBases;

use crate::spells_index::SPELLS_INDEX;

pub trait BuildUi {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait BuildUiNamed {
    fn ui(&mut self, ui: &mut egui::Ui, name: &str);
}

pub trait Identified {
    fn identified(&self) -> bool;
}

macro_rules! impl_identified {
    ($($item: ident),+) => {
        $(
            impl Identified for $item {
                fn identified(&self) -> bool {
                    !self.name.is_empty()
                }
            }
        )+
    }
}

impl_identified!(AttrAPCost, Kampftechnik, Schrift, Sprache);

impl Identified for CharakterTalent {
    fn identified(&self) -> bool {
        !self.base.to_talent_base().name.is_empty()
    }
}

impl BuildUi for Character {
    fn ui(self: &mut Character, ui: &mut egui::Ui) {
        ui.strong("Character");
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::Grid::new("character-grid-1").show(ui, |ui| {
                        ui.label("Name");
                        text_edit!(ui, &mut self.identity.name, 180.0);
                        ui.end_row();

                        ui.label("Geschlecht");
                        self.identity.gender.ui(ui);
                        ui.end_row();

                        ui.label("Erfahrungsgrad");
                        self.erfahrungsgrad.ui(ui);
                        ui.end_row();

                        ui.label("Spezies");
                        self.identity.species.ui(ui);
                        ui.end_row();

                        ui.label("Kultur");
                        text_edit!(ui, &mut self.identity.culture, 180.0);
                        ui.end_row();

                        ui.label("Profession");
                        // ui.horizontal(|ui| {
                        self.identity.profession.ui(ui);
                        // });
                        ui.end_row();

                        ui.label("Sozialstatus");
                        text_edit!(ui, &mut self.identity.social_rank);
                        ui.end_row();

                        ui.label("Heimatort");
                        text_edit!(ui, &mut self.identity.hometown);
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    egui::Grid::new("character-grid-2").show(ui, |ui| {
                        ui.label("Alter / Geburtsdatum");
                        text_edit!(ui, &mut self.identity.age_date_of_birth, 180.0);
                        ui.end_row();

                        ui.label("Haarfarbe");
                        text_edit!(ui, &mut self.identity.hair_color, 180.0);
                        ui.end_row();

                        ui.label("Augenfarbe");
                        text_edit!(ui, &mut self.identity.eye_color, 180.0);
                        ui.end_row();

                        ui.label("Größe");
                        text_edit!(ui, &mut self.identity.size, 180.0);
                        ui.end_row();

                        ui.label("Gewicht");
                        text_edit!(ui, &mut self.identity.weight, 180.0);
                        ui.end_row();

                        ui.label("Familie");
                        text_edit!(ui, &mut self.identity.family, 180.0);
                        ui.end_row();

                        ui.label("Charakteristika");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // text_edit!(ui, &mut self.identity.characteristical, 180.0);
                            ui.text_edit_multiline(&mut self.identity.characteristical);
                        });
                    });
                });
            });
        });
    }
}

impl BuildUi for Gender {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            egui::ComboBox::from_id_source("gender")
                .selected_text(match self {
                    Gender::Female => "Weiblich",
                    Gender::Male => "Männlich",
                    Gender::Diverse(_) => "Divers",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(self, Gender::Female, "Weiblich");
                    ui.selectable_value(self, Gender::Male, "Männlich");
                    ui.selectable_value(self, Gender::Diverse(String::new()), "Divers")
                });
            match self {
                Gender::Diverse(s) => _ = text_edit!(ui, s, 80.0),
                _ => (),
            }
        });
    }
}

// impl BuildUi for Species {
impl BuildUi for crate::spezies::Spezies {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_source("species")
            .selected_text(self.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    self,
                    Spezies::Achaz(Default::default()),
                    achaz::Achaz::name(),
                );
                ui.selectable_value(
                    self,
                    Spezies::Elfen(Default::default()),
                    elfen::Elfen::name(),
                );
                ui.selectable_value(
                    self,
                    Spezies::Halbelfen(Default::default()),
                    halbelfen::Halbelfen::name(),
                );
                ui.selectable_value(
                    self,
                    Spezies::Halborks(Default::default()),
                    halborks::Halborks::name(),
                );
                ui.selectable_value(
                    self,
                    Spezies::Holberker(Default::default()),
                    holberker::Holberker::name(),
                );
                ui.selectable_value(
                    self,
                    Spezies::Menschen(Default::default()),
                    menschen::Menschen::name(),
                );
                ui.selectable_value(self, Spezies::Orks(Default::default()), orks::Orks::name());
                ui.selectable_value(
                    self,
                    Spezies::Zwerge(Default::default()),
                    zwerge::Zwerge::name(),
                );
            });

        crate::widgets::infobutton::Infobutton::new().show_ui(ui, |ui| {
            egui::Grid::new("spezies-details")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    ui.strong("LE");
                    ui.label(format!("{}", self.le()));
                    ui.end_row();
                    ui.strong("SK");
                    ui.label(format!("{}", self.sk()));
                    ui.end_row();
                    ui.strong("ZK");
                    ui.label(format!("{}", self.zk()));
                    ui.end_row();
                    ui.strong("GS");
                    ui.label(format!("{}", self.gs()));
                    ui.end_row();
                    ui.strong("AP-Wert");
                    ui.label(format!("{}", self.cost()));
                    ui.end_row();
                });
        });
    }
}

impl BuildUi for Profession {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Profession Anpassen")
                .open(&mut self.show_editor)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        drag_val!(ui, "AP Kosten", &mut self.ap_cost);
                        // Check if accumulated AP cost is lower or equal to given AP cost
                        let profession_stats_calculated_cost =
                            self.specials.iter().fold(0, |acc, e| acc + e.ap_cost)
                                + self.fighting.iter().fold(0, |acc, e| {
                                    acc + (e.steigerungs_faktor.cost(e.stufe, true))
                                })
                                + self.talents.iter().fold(0, |acc, e| {
                                    acc + (e
                                        .base
                                        .to_talent_base()
                                        .steigerungs_faktor
                                        .cost(e.stufe, true))
                                });
                        let preconditions_cost =
                            self.preconditions.iter().fold(0, |acc, e| acc + e.ap_cost);
                        let calculated_cost = preconditions_cost + profession_stats_calculated_cost;
                        let cost_text = format!("Kosten: {calculated_cost} ({preconditions_cost} + {profession_stats_calculated_cost})");
                        if profession_stats_calculated_cost > self.ap_cost {
                            ui.colored_label(egui::Color32::RED, cost_text);
                        } else {
                            ui.colored_label(egui::Color32::GREEN, cost_text);
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, true])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            collapsing_list(
                                ui,
                                "Voraussetzungen",
                                "profession-edit-grid",
                                &mut self.preconditions,
                                |ui, _, p| {
                                    ui.label("Voraussetzung");
                                    text_edit!(ui, &mut p.name, 180.0);
                                    ui.end_row();
                                    drag_val!(ui, "AP Kosten", &mut p.ap_cost);
                                },
                            );
                            collapsing_list(
                                ui,
                                "Sonderfertigkeiten",
                                "specials-edit-grid",
                                &mut self.specials,
                                |ui, _, p| {
                                    ui.label("Fertigkeit");
                                    text_edit!(ui, &mut p.name, 180.0);
                                    ui.end_row();
                                    drag_val!(ui, "AP Kosten", &mut p.ap_cost);
                                },
                            );
                            collapsing_list(
                                ui,
                                "Kampftechniken",
                                "fighting-edit-grid",
                                &mut self.fighting,
                                |ui, _, p| {
                                    ui.label("Technik");
                                    text_edit!(ui, &mut p.name, 180.0);
                                    ui.end_row();
                                    drag_val!(ui, "Level", &mut p.stufe);
                                    ui.end_row();
                                    p.steigerungs_faktor
                                        .ui(ui, format!("stf-{}", p.name).as_str());
                                    ui.end_row();
                                    ui.label("Leiteigenschaft");
                                    p.leiteigenschaft
                                        .ui(ui, format!("leiteigenschaft-{}", p.name).as_str());
                                },
                            );
                            collapsing_list(
                                ui,
                                "Talente",
                                "profession-talents-edit-grid",
                                &mut self.talents,
                                |ui, id, p| {
                                    let talent_base = p.base.to_talent_base();
                                    ui.label("Name");
                                    egui::ComboBox::from_id_source(format!(
                                        "profession-talent-choose-{}",
                                        id
                                    ))
                                    .selected_text(talent_base.name)
                                    .width(180.0)
                                    .show_ui(ui, |ui| {
                                        for base in enum_iterator::all::<CharakterTalentBases>() {
                                            match base {
                                                CharakterTalentBases::FLIEGEN => {
                                                    ui.label("Körpertalente");
                                                    ui.separator();
                                                }
                                                CharakterTalentBases::BEKEHREN => {
                                                    ui.label("Gesellschaftstalente");
                                                    ui.separator();
                                                }
                                                CharakterTalentBases::FAERTENSUCHEN => {
                                                    ui.label("Naturtalente");
                                                    ui.separator();
                                                }
                                                CharakterTalentBases::BRETTSPIEL => {
                                                    ui.label("Wissenstalente");
                                                    ui.separator();
                                                }
                                                CharakterTalentBases::ALCHIMIE => {
                                                    ui.label("Handwerkstalente");
                                                    ui.separator();
                                                }
                                                _ => (),
                                            }
                                            ui.selectable_value(
                                                &mut p.base,
                                                base.clone(),
                                                base.to_talent_base().name,
                                            );
                                            if matches!(
                                                base,
                                                CharakterTalentBases::ZECHEN
                                                    | CharakterTalentBases::WILLENSKRAFT
                                                    | CharakterTalentBases::WILDNISLEBEN
                                                    | CharakterTalentBases::STERNKUNDE
                                            ) {
                                                ui.separator();
                                            }
                                        }
                                    });
                                    ui.end_row();
                                    drag_val!(ui, "Level", &mut p.stufe);
                                    ui.end_row();
                                    ui.label("StF");
                                    ui.horizontal(|ui| {
                                        ui.label(talent_base.steigerungs_faktor.to_string());
                                        ui.label(format!(
                                            "({})",
                                            talent_base.steigerungs_faktor.cost(p.stufe, true)
                                        ));
                                    });
                                    ui.end_row();
                                    ui.label("Probe");
                                    ui.horizontal(|ui| {
                                        for probe in talent_base.probe {
                                            ui.label(probe.to_string());
                                        }
                                    });
                                },
                            );

                            ui.collapsing("Zaubertrick", |ui| {
                                egui::Grid::new("profession-edit-zauberrick").show(ui, |ui| {
                                    ui.label("Name");
                                    text_edit!(ui, &mut self.zaubertrick.name, 180.0);
                                    ui.end_row();
                                    ui.label("Beschreibung");
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            ui.text_edit_multiline(&mut self.zaubertrick.description);
                                        })
                                    })
                                });
                            });

                        });
                });
        ui.horizontal(|ui| {
            text_edit!(ui, &mut self.name);
            if ui.button("Anpassen").clicked() {
                self.show_editor = !self.show_editor;
            }
        });
    }
}

impl BuildUiNamed for SteigerungsFaktor {
    fn ui(&mut self, ui: &mut egui::Ui, name: &str) {
        ui.label("StF");
        egui::ComboBox::from_id_source(name)
            .selected_text(self.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(self, SteigerungsFaktor::A, SteigerungsFaktor::A.to_string());
                ui.selectable_value(self, SteigerungsFaktor::B, SteigerungsFaktor::B.to_string());
                ui.selectable_value(self, SteigerungsFaktor::C, SteigerungsFaktor::C.to_string());
                ui.selectable_value(self, SteigerungsFaktor::D, SteigerungsFaktor::D.to_string());
            });
    }
}

impl BuildUiNamed for AttrType {
    fn ui(&mut self, ui: &mut egui::Ui, name: &str) {
        egui::ComboBox::from_id_source(name)
            .selected_text(self.to_string_long())
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Self::Any, Self::Any.to_string_long());
                ui.selectable_value(self, Self::MU, Self::MU.to_string_long());
                ui.selectable_value(self, Self::KL, Self::KL.to_string_long());
                ui.selectable_value(self, Self::IN, Self::IN.to_string_long());
                ui.selectable_value(self, Self::CH, Self::CH.to_string_long());
                ui.selectable_value(self, Self::FF, Self::FF.to_string_long());
                ui.selectable_value(self, Self::GE, Self::GE.to_string_long());
                ui.selectable_value(self, Self::KO, Self::KO.to_string_long());
                ui.selectable_value(self, Self::KK, Self::KK.to_string_long());
            });
    }
}

impl ZauberTable {
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("Zauber Auswählen")
            .open(&mut self.show_selector)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Zauber Suchen: ");
                        ui.text_edit_singleline(&mut self.search);
                    });

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .id_source("spell-select-scroll")
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                egui::Grid::new("spell-select-grid").show(ui, |ui| {
                                    for (id, spell) in SPELLS_INDEX.into_iter().enumerate() {
                                        if !self.search.is_empty()
                                            && !spell.name.starts_with(&self.search)
                                        {
                                            continue;
                                        }

                                        let mut enable_spell =
                                            self.enabled_spells.contains_key(&id);
                                        let mut checkbox_text = egui::RichText::new(spell.name);
                                        if enable_spell {
                                            checkbox_text =
                                                checkbox_text.color(egui::Color32::GREEN);
                                        }
                                        if ui.checkbox(&mut enable_spell, checkbox_text).clicked() {
                                            if enable_spell {
                                                self.enabled_spells
                                                    .insert(id, ZauberDescriptor::default());
                                            } else {
                                                self.enabled_spells.remove(&id);
                                            }
                                        }
                                        probe_display(ui, spell);
                                        ui.label(format!("StF: {}", spell.stf.to_string(),));
                                        ui.label(
                                            spell
                                                .verbreitung
                                                .iter()
                                                .map(ToString::to_string)
                                                .collect::<Vec<_>>()
                                                .join(", "),
                                        );
                                        ui.end_row();
                                    }
                                });
                            });
                        });
                });
            });
    }

    pub fn full_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Zauber Auswählen").clicked() {
            self.show_selector = !self.show_selector;
        }
        for (id, spell_desc) in &mut self.enabled_spells {
            let spell = SPELLS_INDEX[*id];
            // let collapsing_id = ui.make_persistent_id(format!("collapsing-spell-{}", spell.name));
            // let (_, header_response, _) =
            // let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
            // ui.ctx(),
            // collapsing_id,
            // false,
            // );
            // .show_header(ui, |ui| {
            // let header_res = ui.horizontal(|ui| {
            // let button_response =
            // state.show_toggle_button(ui, egui::collapsing_header::paint_default_icon);
            // let label_response = ui.add(
            // egui::Label::new(egui::RichText::new(spell.name).heading())
            // .sense(egui::Sense::click()),
            // );
            // (button_response, label_response)
            // });
            // state.show_body_indented(&header_res.response, ui, |ui| {
            ui.collapsing(egui::RichText::new(spell.name).heading(), |ui| {
                ui.label(spell.wirkung.replace(". ", ".\n"));
                egui::Grid::new("spell-grid-{id}").show(ui, |ui| {
                    ui.strong("Level: ");
                    drag_val!(ui, &mut spell_desc.level);
                    ui.end_row();
                    ui.strong("StF: ");
                    ui.label(spell.stf.to_string());
                    ui.end_row();
                    ui.strong("Probe: ");
                    probe_display(ui, spell);
                    if let Some(probe_modifikator) = &spell.probe_modifikator {
                        ui.label(format!(
                            "Modifiziert durch {}",
                            probe_modifikator.to_string()
                        ));
                    }
                    ui.end_row();
                    ui.strong("Dauer: ");
                    ui.label(format!("{} Aktionen", spell.dauer));
                    if !spell.dauer_modifiable {
                        ui.label("Dauer kann nicht modifiziert werden");
                    }
                    ui.end_row();
                    ui.strong("ASP Kosten: ");
                    ui.label(format!("{}", spell.asp.to_string()));
                    if !spell.asp_modifiable {
                        ui.label("Kosten können nicht modifiziert werden");
                    }
                    ui.end_row();
                    ui.strong("Reichweite: ");
                    ui.label(format!("{}", spell.reichweite.to_string()));
                    if !spell.reichweite_modifiable {
                        ui.label("Reichweite kann nicht modifiziert werden");
                    }
                    ui.end_row();
                    ui.strong("Wirkungsdauer: ");
                    ui.label(spell.wirkungsdauer);
                    ui.end_row();
                    ui.strong("Zielkategorie: ");
                    ui.label(spell.zielkategorie.to_string());
                    ui.end_row();
                    ui.strong("Merkmal: ");
                    ui.label(spell.merkmal.to_string());
                    ui.end_row();
                    ui.strong("Verbreitung: ");
                    ui.label(
                        spell
                            .verbreitung
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(", "),
                    );
                });
                ui.collapsing(egui::RichText::new("Erweiterungen: ").strong(), |ui| {
                    for (ext_id, erweiterung) in spell.erweiterungen.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let mut ext_enabled = spell_desc.enabled_extensions.contains(&ext_id);
                            if ui
                                .add(egui::Checkbox::without_text(&mut ext_enabled))
                                .clicked()
                            {
                                if ext_enabled {
                                    spell_desc.enabled_extensions.insert(ext_id);
                                } else {
                                    spell_desc.enabled_extensions.remove(&ext_id);
                                }
                            }
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(erweiterung.name);
                                    ui.label(format!(
                                        "(FW {}, {} AP)",
                                        erweiterung.fw, erweiterung.ap
                                    ));
                                });
                                // ui.horizontal(|ui| {
                                // ui.label(erweiterung.desc);
                                // });
                                ui.add(egui::Label::new(erweiterung.desc).wrap(true));
                            });
                        });
                        ui.separator();
                    }
                });
            });
            // if header_res.inner.1.clicked() {
            // state.toggle(ui);
            // state.store(ui.ctx());
            // }
            ui.separator();
        }
    }
}

fn probe_display(ui: &mut egui::Ui, spell: &crate::data::Zauber) {
    ui.label(format!(
        "{}/{}/{}",
        spell.probe[0].to_string(),
        spell.probe[1].to_string(),
        spell.probe[2].to_string(),
    ));
}

pub fn collapsing_list<T>(
    ui: &mut egui::Ui,
    collapsing_label: &str,
    grid_name: &str,
    items: &mut Vec<T>,
    mut add_content: impl FnMut(&mut egui::Ui, u32, &mut T),
) where
    T: Default + Identified,
{
    ui.collapsing(collapsing_label, |ui| {
        // ui.label("Voraussetzungen");
        let mut id = 0;
        ui.vertical(|ui| {
            let mut empty_item = false;
            items.retain_mut(|item| {
                // Check if the item has a unique identifier, eg. non empty name
                if !item.identified() {
                    // If there is another unidentified item already, skip this one
                    // otherwise remember this
                    if empty_item {
                        return false;
                    } else {
                        empty_item = true;
                    }
                }

                let mut retain = true;
                ui.horizontal(|ui| {
                    egui::Grid::new(format!("{}-{}", grid_name, id)).show(ui, |ui| {
                        add_content(ui, id, item);
                    });
                    retain = !ui.button("x").clicked()
                });
                ui.separator();
                id += 1;
                retain
            });
            // Don't allow creating another entry if there is an unidentified entry
            // This might be redundant, but the previous check does prevent
            // adding empty entries in other ways
            if ui
                .add_enabled(!empty_item, egui::Button::new("+"))
                .on_disabled_hover_ui(|ui| {
                    ui.label("Leerer Eintrag!");
                })
                .clicked()
            {
                items.push(Default::default());
            }
        });
    });
}
