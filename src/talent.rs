use crate::data::Talent;

pub fn talent_ui(ui: &mut egui::Ui, description: &str, talents: &mut Vec<Talent>) {
    ui.label(description);
    ui.vertical(|ui| {
        (&mut *talents).retain_mut(|talent| {
            let mut keep_talent = true;
            ui.horizontal(|ui| {
                text_edit!(ui, &mut talent.name);
                egui::TextEdit::multiline(&mut talent.description).show(ui);
                if ui.button("X").clicked() {
                    keep_talent = false;
                }
            });
            keep_talent
        });
        if ui.button("+").clicked() {
            talents.push(Talent::default());
        }
    });
}
