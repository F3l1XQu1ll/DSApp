use std::collections::BTreeMap;

pub struct CharacterTalentsView<'a> {
    talent_searched: &'a mut String,
    talents: &'a mut BTreeMap<usize, u16>,
}

impl<'a> CharacterTalentsView<'a> {
    pub fn new(talent_searched: &'a mut String, talents: &'a mut BTreeMap<usize, u16>) -> Self {
        Self {
            talent_searched,
            talents,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        use crate::character_talents::TalentClass;
        ui.horizontal(|ui| {
            ui.label("Suchen: ");
            text_edit!(ui, self.talent_searched);
        });
        ui.separator();
        let mut scroll_to_talent: Option<&'static crate::data::CharakterTalentBase> = None;
        let mut scroll_to_class: Option<crate::character_talents::TalentClass> = None;
        if !self.talent_searched.is_empty() {
            (scroll_to_class, scroll_to_talent) = enum_iterator::all::<TalentClass>()
                .find_map(|id| {
                    id.talents_iter()
                        .find(|&&t| {
                            t.name
                                .to_lowercase()
                                .starts_with(self.talent_searched.to_lowercase().as_str())
                        })
                        .map(|t| (id.clone(), *t))
                })
                .unzip();
        }

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for class_id in enum_iterator::all::<TalentClass>() {
                    let class_talents = class_id.talents_iter();
                    egui::CollapsingHeader::new(
                        egui::RichText::new(class_id.description()).heading(),
                    )
                    .open(scroll_to_class.as_ref().map(|c| *c == class_id))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for (index, talent) in class_talents.enumerate() {
                                if let Some(scroll_target) = scroll_to_talent {
                                    if talent.name != scroll_target.name {
                                        continue;
                                    }
                                }
                                ui.heading(talent.name);
                                egui::Grid::new(format!("talent-{}", talent.name)).show(ui, |ui| {
                                    ui.strong("Probe");
                                    ui.label(
                                        talent
                                            .probe()
                                            .iter()
                                            .map(|p| p.to_string())
                                            .collect::<Vec<_>>()
                                            .join("/"),
                                    );
                                    ui.end_row();
                                    let talent_level = self
                                        .talents
                                        .entry(class_id.talent_index(index))
                                        .or_insert(1);
                                    drag_val!(ui, "Level", talent_level);
                                    ui.end_row();
                                    let cost = if *talent_level > 1 {
                                        talent.steigerungs_faktor.cost(*talent_level, false)
                                    } else {
                                        0
                                    };
                                    ui.strong("Kosten");
                                    ui.label(format!(
                                        "{} AP ({})",
                                        cost,
                                        talent.steigerungs_faktor.to_string()
                                    ));
                                });
                                ui.separator();
                            }
                        });
                    });
                    ui.separator();
                }
            });
    }
}
