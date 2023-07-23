use crate::display::{collapsing_list, BuildUi};

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

// impl BuildUi for Sprachen und Schriften
impl BuildUi for SprachenSchriften {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Sprachen und Schriften")
            .open(&mut self.show_editor)
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Sprachen
                            collapsing_list(
                                ui,
                                "Sprachen",
                                "sprache-add-grid",
                                &mut self.sprachen,
                                |ui, _, p| {
                                    text_edit!(ui, &mut p.name, 180.0);
                                    ui.label("");
                                    p.stufe.ui(ui);
                                    ui.label("");
                                    //ui.label("Stufe");
                                    //egui::ComboBox::from_id_source("species")
                                    //    .selected_text(self.name())
                                    //    .show_ui(ui, |ui| {
                                    //        ui.selectable_value(self, Stufe::I, "I");
                                    //    });
                                    //oder
                                    //text_edit!(ui, &mut p.level, 180.0);
                                    drag_val!(ui, "AP", &mut p.cost);
                                    ui.end_row();
                                },
                            );
                            // Schriften
                            collapsing_list(
                                ui,
                                "Schriften",
                                "sprache-add-grid",
                                &mut self.schriften,
                                |ui, _, p| {
                                    //ui.label("Sprache");
                                    text_edit!(ui, &mut p.name, 180.0);
                                    ui.label("");
                                    drag_val!(ui, "AP", &mut p.cost);
                                    ui.end_row();
                                },
                            );
                        });
                    });
            });
        ui.horizontal(|ui| {
            if ui.button("Sprachen und Schriften").clicked() {
                self.show_editor = !self.show_editor;
            }
        });
    }
}

impl BuildUi for SpracheStufe {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            egui::ComboBox::from_id_source("gender")
                .selected_text(match self {
                    SpracheStufe::I => "I",
                    SpracheStufe::II => "II",
                    SpracheStufe::III => "III",
                    SpracheStufe::MS => "MS",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(self, SpracheStufe::I, "I");
                    ui.selectable_value(self, SpracheStufe::II, "II");
                    ui.selectable_value(self, SpracheStufe::III, "III");
                    ui.selectable_value(self, SpracheStufe::MS, "MS");
                });
        });
    }
}
