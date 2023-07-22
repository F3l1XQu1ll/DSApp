use egui::Ui;

use crate::data::Vorteil;

#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
#[serde(default)]
pub struct VorteileView {
    vorteile: Vec<Vorteil>,
    nachteile: Vec<Vorteil>,
    show_window: bool,
}

impl VorteileView {
    pub fn window_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Vor-/Nachteile").clicked() {
                self.show_window = !self.show_window;
            }
        });
        egui::Window::new("Vor-/Nachteile")
            .open(&mut self.show_window)
            .show(ui.ctx(), |ui| {
                ui.collapsing("Vorteile", |ui| {
                    egui::ScrollArea::new([false, true])
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            Self::add_vorteil_list(ui, &mut self.vorteile);
                        });
                });
                ui.collapsing("Nachteile", |ui| {
                    egui::ScrollArea::new([false, true])
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            Self::add_vorteil_list(ui, &mut self.nachteile);
                        });
                });
            });
    }
    fn add_vorteil_list(ui: &mut Ui, vorteile: &mut Vec<Vorteil>) {
        vorteile.retain_mut(|vorteil| {
            egui::Grid::new("vorteile-edit-grid").show(ui, |ui| {
                ui.strong("Name");
                text_edit!(ui, &mut vorteil.name, 180.0);
                ui.end_row();
                ui.strong("Beschreibung");
                egui::TextEdit::multiline(&mut vorteil.beschreibung).show(ui);
                ui.end_row();
                ui.strong("Stufe");
                drag_val!(ui, &mut vorteil.stufe);
                ui.end_row();
                ui.strong("AP");
                drag_val!(ui, &mut vorteil.ap);
            });

            !ui.button("x").clicked()
        });
        if matches!(
            vorteile.last().map(|vorteil| vorteil.name.is_empty()),
            Some(false) | None
        ) {
            if ui.button("+").clicked() {
                vorteile.push(Vorteil::default());
            }
        }
    }
}
