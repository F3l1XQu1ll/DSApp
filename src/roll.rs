use std::{default, string};

use egui::Id;

use crate::debugger;

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum ShowRollEditor {
    Roll,
    Modifier,
    #[default]
    None,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Roll {
    pub show_editor: ShowRollEditor,
    pub mod_level: i8,    //modifier for value to balance rolls
    pub mod_roll: i8,     //modifer for value roll again
    pub mod_boni: i8,     //modifier for roll result
    pub rolld6: Vec<u8>,  //d6s rolled
    pub rolld20: Vec<u8>, //d20s rolled
    pub passed: bool,
    pub krit: Krit,
}

// kritische Würfe
#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Krit {
    Patzer,
    #[default]
    None,
    KritischerErfolg,
}

impl Roll {
    // ------------ Kampf -----------------
    pub fn ui_kampf_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("würfeln").clicked() {
                self.show_editor = ShowRollEditor::Roll;
            }
        });
    }
    pub fn ui_kampf_window(&mut self, ui: &mut egui::Ui, skill_name: Id) {
        egui::Window::new("Wurf")
            .open(&mut { self.show_editor == ShowRollEditor::Roll })
            .id(skill_name)
            .show(ui.ctx(), |ui| {
                ui.group(|ui| {
                    self.ui_kampf_mod(ui, skill_name);
                    if ui.button("x").clicked() {
                        self.show_editor = ShowRollEditor::None;
                    }
                });
            });
    }

    pub fn ui_kampf_mod(&mut self, ui: &mut egui::Ui, skill_name: Id) {
        egui::Window::new("Modifier")
            .open(&mut { self.show_editor == ShowRollEditor::Modifier })
            .id(skill_name)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Erschwernis / Erleichterung");
                        drag_val_mod!(ui, &mut self.mod_roll);
                    });
                });
            });
        ui.horizontal(|ui| {
            if ui.button("mali/boni").clicked() {
                self.show_editor = ShowRollEditor::Modifier;
            }
        });
    }
}
