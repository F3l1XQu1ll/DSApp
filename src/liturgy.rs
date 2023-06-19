use crate::data::CostType;
use crate::data::Liturgy;
use crate::data::Range;
use crate::data::SteigerungsFaktor;
use crate::display::BuildUiNamed;

pub fn build_liturgies_table(ctx: &egui::Context, ui: &mut egui::Ui, liturgies: &mut Vec<Liturgy>) {
    egui::ScrollArea::vertical()
        .id_source("liturgies-table")
        .show(ui, |ui| {
            let table = egui_extras::TableBuilder::new(ui)
                .columns(egui_extras::Column::auto().resizable(true), 12);
            table
                .header(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Liturgie / Zeremonie");
                    });
                    row.col(|ui| {
                        ui.label("Probe");
                    });
                    row.col(|ui| {
                        ui.label("Fw");
                    });
                    row.col(|ui| {
                        ui.label("Kosten");
                    });
                    row.col(|ui| {
                        ui.label("Liturgiedauer");
                    });
                    row.col(|ui| {
                        ui.label("Reichweite");
                    });
                    row.col(|ui| {
                        ui.label("Wirkunksdauer");
                    });
                    row.col(|ui| {
                        ui.label("Aspekt");
                    });
                    row.col(|ui| {
                        ui.label("StF");
                    });
                    row.col(|ui| {
                        ui.label("Wirkung");
                    });
                    row.col(|ui| {
                        ui.label("S.");
                    });
                })
                .body(|mut body| {
                    let mut id: usize = 0;
                    (&mut *liturgies).retain_mut(|liturgy| {
                        let Liturgy {
                            // id,
                            name,
                            probe,
                            fw,
                            cost,
                            duration,
                            aspect,
                            stf,
                            effect,
                            s,
                            range,
                            effect_duration,
                            effect_name,
                            show_editor,
                        } = liturgy;
                        let mut keep_row = false;
                        id += 1;

                        body.row(18.0, |mut row| {
                            // Liturgie
                            row.col(|ui| {
                                text_edit!(ui, name);
                            });
                            // Probe
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for (idx, attr) in probe.0.iter_mut().enumerate() {
                                        attr.ui(ui, &format!("probe-{idx}"));
                                        // ui.label(attr.to_string());
                                    }
                                });
                            });
                            // Fw
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(fw));
                            });
                            // Kosten
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add(egui::DragValue::new(&mut cost.amount));
                                    egui::ComboBox::from_id_source(format!("cost-type-{}", id))
                                        .selected_text(cost.costs_type.to_string())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut cost.costs_type,
                                                CostType::AsP,
                                                CostType::AsP.to_string(),
                                            )
                                        });
                                });
                            });
                            // Liturgiedauer
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(duration));
                            });
                            // Reichweite
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    match range {
                                        Range::Foot(foot) => {
                                            ui.add(egui::DragValue::new(foot));
                                        }
                                        _ => (),
                                    }
                                    egui::ComboBox::from_id_source(format!("range-{}", id))
                                        .selected_text(match range {
                                            Range::CastOnSelf => "Selbst",
                                            Range::Foot(_) => "Fuß",
                                            Range::Touch => "Berührung",
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(range, Range::CastOnSelf, "Selbst");
                                            ui.selectable_value(range, Range::Foot(0), "Fuß");
                                        });
                                });
                            });
                            // Wirkungsdauer
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(effect_duration));
                            });
                            // Aspekt
                            row.col(|ui| {
                                ui.text_edit_singleline(aspect);
                            });
                            // StF
                            row.col(|ui| {
                                egui::ComboBox::from_id_source(format!("StF-{}", id))
                                    .selected_text(stf.to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(stf, SteigerungsFaktor::A, "A");
                                        ui.selectable_value(stf, SteigerungsFaktor::B, "B");
                                        ui.selectable_value(stf, SteigerungsFaktor::C, "C");
                                    });
                            });
                            // Wirkung
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("Bearbeiten").clicked() {
                                        *show_editor = true;
                                    }
                                    ui.label(effect_name.as_str())
                                        .on_hover_text(effect.as_str());
                                    egui::Window::new("Litugie Effekt Bearbeiten")
                                        .id(egui::Id::new(format!(
                                            "litrgy-{}-effect-edit-window",
                                            id
                                        )))
                                        .open(show_editor)
                                        .show(ctx, |ui| {
                                            text_edit!(ui, effect_name);
                                            egui::ScrollArea::vertical().id_source(id).show(
                                                ui,
                                                |ui| {
                                                    egui::TextEdit::multiline(effect).show(ui);
                                                },
                                            );
                                        });
                                });
                            });
                            // S.
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(s));
                            });
                            row.col(|ui| {
                                if ui.button("X").clicked() {
                                    keep_row = false;
                                } else {
                                    keep_row = true;
                                }
                            });
                        });
                        keep_row
                    });
                });
        });
}
