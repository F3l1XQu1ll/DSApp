use std::{borrow::Cow, rc::Rc};

use crate::{
    data::{Animal, Archetype, Attributes, Character, Equipment, KaP, Liturgy, Talent},
    display::BuildUi,
    drag_val, get, gt, ld, mul,
    properties::{Check, Operation, Value},
    spezies::SpeziesBase,
    sprachen_schriften::{Schrift, Sprache, SprachenSchriften},
    sub,
    talent::talent_ui,
    text_edit,
    views::View,
};

/// Contains all state that belongs to the character sheet.
/// Other data may be stored in [`egui`](https://docs.rs/egui/latest/egui/)s [`Context`](egui::Context),
/// however we mostly don't care about that.
#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct DSApp {
    // Internae
    /// Stores the current [`ron`](ron) version of [`DSApp`](crate::DSApp), ommiting this field
    #[serde(skip)]
    data_code: String,
    /// Should we reconstruct the state from ron?
    /// Also not stored in `data_code`.
    #[serde(skip)]
    replace_data: bool,
    /// Should the ron code that is stored in `data_code`
    /// be automatically updated in the debugger?
    #[serde(skip)]
    autoupdate_code: bool,
    /// Stores the input for the talents search box
    /// We don't care to persist this either
    #[serde(skip)]
    talent_searched: String,
    // Character attributes
    /// Kamalpunkte
    #[deprecated]
    kap: KaP, // TODO Remove in favour of new ui
    /// Astralpunkte
    #[deprecated]
    asp: KaP, // FIXME Hack, TODO New ui, new calc
    /// Lebenspunkte(?)
    /// Copied from sheet …
    #[deprecated]
    le: KaP, // FIXME Hack, TODO New ui, new calc
    /// Stores the current character
    character: Character,
    /// Character main attributes
    /// `DEPRECATED!` use `character.attributes` instead.
    #[deprecated]
    attributes: Attributes,
    /// Liturgien UND Zauber
    /// currently not displayed in the UI due to planned rework
    #[deprecated]
    liturgies: Vec<Liturgy>, // TODO Rework
    #[deprecated]
    tradition: String,
    #[deprecated]
    main_trait: String,
    #[deprecated]
    aspect: String,
    #[deprecated]
    feature: String,
    #[deprecated]
    talents: Vec<Talent>,
    #[deprecated]
    clerics: Vec<Talent>,
    #[deprecated]
    ceremonial: Talent,
    #[deprecated]
    blessings: Vec<Talent>,
    #[deprecated]
    abilities: Vec<Talent>,
    #[deprecated]
    artifact: Talent,
    #[deprecated]
    magic: Vec<Talent>,
    #[deprecated]
    archetype: Archetype,
    #[deprecated]
    equipment: Vec<Equipment>,
    #[deprecated]
    buoyancy: u16, // Tragkraft
    #[deprecated]
    dukaten: u16,
    #[deprecated]
    silber: u16,
    #[deprecated]
    heller: u16,
    #[deprecated]
    kreuzer: u16,
    #[deprecated]
    animal: Animal,
    #[deprecated]
    sprachenschriften: SprachenSchriften,

    // Properties und Operationen
    store: crate::properties::PropertiesManager,
}

// impl Default for DSApp {
//     fn default() -> Self {
//         Self {
//             data_code: String::new(),
//             replace_data: false,
//             autoupdate_code: false,
//             talent_searched: String::new(),
//             kap: KaP::default(),
//             asp: KaP::default(),
//             le: KaP::default(),
//             character: Character::default(),
//             attributes: Attributes::default(),
//             liturgies: Vec::new(),
//             tradition: String::default(),
//             main_trait: String::default(),
//             aspect: String::default(),
//             talents: Vec::new(),
//             clerics: Vec::new(),
//             ceremonial: Talent::default(),
//             blessings: Vec::new(),
//             feature: String::default(),
//             abilities: Vec::new(),
//             artifact: Talent::default(),
//             magic: Vec::new(),
//             archetype: Archetype::default(),
//             equipment: Vec::new(),
//             buoyancy: 0,
//             dukaten: 0,
//             silber: 0,
//             heller: 0,
//             kreuzer: 0,
//             animal: Animal::default(),
//         }
//     }
// }

impl DSApp {
    /// Only called at startup from `main()` (found in `bin/`)
    /// Loads state from local storage.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Schriften konfigurieren
        let mut style = (*cc.egui_ctx.style()).clone();
        style
            .text_styles
            .values_mut()
            .for_each(|text_style| text_style.size *= 1.25);
        cc.egui_ctx.set_style(style);

        let mut instance: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        crate::properties::strings::register(&mut instance.store);
        instance
    }
}

impl eframe::App for DSApp {
    /// Autosave is set to 5 seconds tick
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Reload state based on text in debug window
        if self.replace_data {
            let data_copy = self.data_code.clone();
            if let Ok(data) = ron::from_str(&data_copy) {
                *self = data;
                self.data_code = data_copy;
            }
            self.replace_data = false;
        }
        // We keep this for later in order to serialize it and show the result in the debugger
        let self_clone = (&*self).clone();
        // Some shorthand init for all our state …
        let Self {
            data_code,
            replace_data,
            autoupdate_code,
            talent_searched,
            kap,
            asp,
            le,
            attributes,
            liturgies,
            tradition,
            main_trait,
            aspect,
            talents,
            clerics,
            ceremonial,
            blessings,
            feature,
            archetype,
            abilities,
            artifact,
            magic,
            equipment,
            buoyancy,
            dukaten,
            silber,
            heller,
            kreuzer,
            animal,
            character,
            store,
            sprachenschriften,
        } = self;
        let Attributes {
            mu,
            kl,
            r#in,
            ch,
            ff,
            ge,
            ko,
            kk,
        } = attributes;

        // Add debugger window to screen
        crate::debugger::Debugger::new(ctx, &self_clone, data_code, autoupdate_code, replace_data)
            .show();

        // Top panel comes first, current layout is:
        // |           TOP          |
        // --------------------------
        // | CENTER | RIGHT | RIGHT |
        //
        // Outmost right is for spells, inner is for talents
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("DSApp");
            egui::warn_if_debug_build(ui);

            // Charakter
            character.ui(ui);
            // Zauber (Fenster), NICHT panel!
            character.spells.ui(ctx, ui);

            // Kamalpunkte (KaP) Max and current
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        drag_val!(ui, "KaP Max", &mut kap.max);
                    });
                    ui.separator();

                    ui.vertical(|ui| {
                        drag_val!(ui, "Aktuell", &mut kap.current);
                    });
                });

                // Astralpukte
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        // FIXME Only regarding mages and specifically witches (charisma leiteigenschaft) sowie Kesselzauber
                        let asp_default_calc =
                            if *archetype == Archetype::Mage { 20 } else { 0 } + *ch as u16;
                        asp.max = if asp_default_calc >= 2 {
                            asp_default_calc - 2
                        } else {
                            0
                        };
                        ui.strong("AsP Max");
                        ui.strong(format!("{}", asp.max));
                    });

                    ui.separator();
                    ui.vertical(|ui| {
                        drag_val!(ui, "Aktuell", &mut asp.current);
                    });
                });

                // Lebenspunkte
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        le.max = character.identity.species.le() as u16
                            + 2 * character.attributes.ko as u16;
                        ui.strong("LeP Max");
                        ui.strong(format!("{}", le.max));
                    });

                    ui.separator();
                    ui.vertical(|ui| {
                        drag_val!(ui, "Aktuell", &mut le.current);
                    });
                });

                // Initiative, Seelenkraft, Zähigkeit
                ui.group(|ui| {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.strong("Ini");
                        let initiative_base = (*mu + *ge) / 2;
                        ui.strong(format!("{initiative_base}"));
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.strong("SK");
                        let sk_base = character.identity.species.sk() as i16
                            + ((*mu + *kl + *r#in) / 6) as i16;
                        ui.strong(format!("{sk_base}"));
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.strong("ZK");
                        let zk_base =
                            character.identity.species.zk() as i16 + ((*ko + *ko + *kk) / 6) as i16;
                        ui.strong(format!("{zk_base}"));
                    });
                });

                // Sprachen, Schriften
                ui.group(|ui| {
                    sprachenschriften.ui(ui);
                    //ui.button("Sprache und Schrift").clicked()
                });

                // Vorteile, Nachteile
                ui.group(|ui| {
                    character.apmods.window_ui(ui);
                });
            });

            // Attributes
            ui.horizontal(|ui| {
                // create new stat calculator for the character
                // must be mutable for ui
                let mut calc = crate::cost::AttributeCalc::new(character);
                ui.vertical(|ui| {
                    ui.strong("MU");
                    ui.add(calc.ui_mu());
                    // drag_val!(ui, "MU", &mut character.attributes.mu);
                });
                ui.vertical(|ui| {
                    ui.strong("KL");
                    ui.add(calc.ui_kl());
                    // drag_val!(ui, "KL", &mut character.attributes.kl);
                });
                ui.vertical(|ui| {
                    ui.strong("IN");
                    ui.add(calc.ui_in());
                    // drag_val!(ui, "IN", &mut character.attributes.r#in);
                });
                ui.vertical(|ui| {
                    ui.strong("CH");
                    ui.add(calc.ui_ch());
                    // drag_val!(ui, "CH", &mut character.attributes.ch);
                });
                ui.vertical(|ui| {
                    ui.strong("FF");
                    ui.add(calc.ui_ff());
                    // drag_val!(ui, "FF", &mut character.attributes.ff);
                });
                ui.vertical(|ui| {
                    ui.strong("GE");
                    ui.add(calc.ui_ge());
                    // drag_val!(ui, "GE", &mut character.attributes.ge);
                });
                ui.vertical(|ui| {
                    ui.strong("KO");
                    ui.add(calc.ui_ko());
                    // drag_val!(ui, "KO", &mut character.attributes.ko);
                });
                ui.vertical(|ui| {
                    ui.strong("KK");
                    ui.add(calc.ui_kk());
                    // drag_val!(ui, "KK", &mut character.attributes.kk);
                });
            });
        });

        // Zauber
        egui::SidePanel::right("panel-spells").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    character.spells.full_ui(ui);
                });
        });

        // Talente
        egui::SidePanel::right("panel-talents").show(ctx, |ui| {
            crate::talents_view::CharacterTalentsView::new(talent_searched, &mut character.talents)
                .show(ui);
        });

        // Kampftechniken
        egui::SidePanel::right("panel-fighting_skills").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    crate::kampf::KampfTechnikenView::new(&mut character.kampftechniken).ui(ui);
                });
        });

        // Derzeit debug für Kosten und Attribut berechnungen
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("AP:");
            crate::cost::CostView::new(character).show(ui);
            ui.label("Name (prop)");
            ui.text_edit_singleline(
                store
                    .properties
                    .entry("/character/name".into())
                    .or_insert(Cow::from("Moin").into())
                    .to_str_mut()
                    .unwrap(),
            );
            ui.label("Mut (prop)");
            ui.add(egui::DragValue::new(
                store
                    .properties
                    .entry("/character/attribute/mu".into())
                    .or_insert(8.0.into())
                    .to_val_mut()
                    .unwrap(),
            ));
            // let mut_kosten_op = mul!(sub!(get!("/character/attribute/mu"), ld!(8.0)), ld!(15.0));
            let mut_kosten_op = gt!(
                get!("/character/attribute/mu"),
                ld!(8.0),
                ld!("Größer"),
                ld!("Kleiner")
            );

            ui.label(format!(
                "Mut kosten: {:?}",
                store
                    .operations
                    .entry("/mut_kosten".into())
                    .or_insert(mut_kosten_op)
                    .eval(&mut store.properties)
                    .and_then(Value::to_str)
                    .unwrap(),
            ));

            // let op = op!(["/character/attribute/mu"] + 2.0);

            // Neues Sheet
            crate::views::character::Character::view(store, ui);
        });

        // Ab hier nur viel altes Zeug, muss noch aktualisiert und neu eingefügt werden
        /*egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // liturgies, ceremonies
                    crate::liturgy::build_liturgies_table(ctx, ui, liturgies);

                    // add new liturgy
                    if ui.button("+").clicked() {
                        liturgies.push(Liturgy::default());
                    }

                    ui.horizontal(|ui| {
                        ui.label("Typus: ");
                        egui::ComboBox::from_id_source("archetype")
                            .selected_text(match archetype {
                                Archetype::Cleric => "Keleriker",
                                Archetype::Mage => "Magier",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(archetype, Archetype::Cleric, "Kleriker");
                                ui.selectable_value(archetype, Archetype::Mage, "Magier");
                            });
                    });

                    egui::Grid::new("tradition-grid")
                        .spacing((10.0, 20.0))
                        .striped(true)
                        .show(ui, |ui| {
                            // tradition
                            ui.label("Tradtion: ");
                            text_edit!(ui, tradition, 200.0);
                            ui.end_row();

                            // main trait
                            ui.label("Leiteigenschaft: ");
                            text_edit!(ui, main_trait, 200.0);
                            ui.end_row();

                            match archetype {
                                Archetype::Cleric => {
                                    // aspect
                                    ui.label("Aspekt: ");
                                    text_edit!(ui, aspect, 200.0);
                                    ui.end_row();

                                    // talents
                                    talent_ui(ui, "Wohlgefällige Talente: ", talents);
                                    ui.end_row();
                                }
                                Archetype::Mage => {
                                    // aspect
                                    ui.label("Merkmal: ");
                                    text_edit!(ui, feature, 200.0);
                                    ui.end_row();
                                }
                            }

                            let cleric_tables = (
                                ("Kleriale Sonderfertigkeiten: ", clerics),
                                ("Zeremoniengegenstand: ", ceremonial),
                                ("Segnungen: ", blessings),
                            );

                            let mage_tables = (
                                ("Magische Sonderfertigkeiten: ", abilities),
                                ("Traditionsartefakt: ", artifact),
                                ("Zaubertricks: ", magic),
                            );

                            {
                                let (
                                    (abilities_desc, abilities),
                                    (artifact_desc, artifact),
                                    (magic_desc, magic),
                                ) = match archetype {
                                    Archetype::Cleric => cleric_tables,
                                    Archetype::Mage => mage_tables,
                                };

                                talent_ui(ui, abilities_desc, abilities);
                                ui.end_row();

                                ui.label(artifact_desc);
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        text_edit!(ui, &mut artifact.name);
                                        egui::TextEdit::multiline(&mut artifact.description)
                                            .show(ui);
                                    });
                                });
                                ui.end_row();

                                talent_ui(ui, magic_desc, magic);
                                ui.end_row();
                            }
                        });

                    ui.add_space(20.0);

                    // equipment table
                    egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .columns(egui_extras::Column::auto(), 4)
                        .header(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Gegenstand");
                            });
                            row.col(|ui| {
                                ui.label("Gewicht");
                            });
                            row.col(|ui| {
                                ui.label("Wo Getragen");
                            });
                        })
                        .body(|mut body| {
                            equipment.retain_mut(|item| {
                                let mut keep_item = true;
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        text_edit!(ui, &mut item.name);
                                    });
                                    row.col(|ui| {
                                        ui.add(egui::DragValue::new(&mut item.weight));
                                    });
                                    row.col(|ui| {
                                        text_edit!(ui, &mut item.location);
                                    });
                                    row.col(|ui| {
                                        if ui.button("x").clicked() {
                                            keep_item = false;
                                        }
                                    });
                                });
                                keep_item
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Gesammtgewicht: ");
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{}",
                                        equipment
                                            .iter()
                                            .map(|e| e.weight)
                                            .reduce(|total, e| total + e)
                                            .unwrap_or_default()
                                    ));
                                });
                                row.col(|ui| {
                                    ui.label("Tragkraft: \n(KKx2)");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(buoyancy));
                                });
                            });
                        });

                    // add new equipment
                    if ui.button("+").clicked() {
                        equipment.push(Equipment::default());
                    }

                    // money
                    egui::Grid::new("money-grid").show(ui, |ui| {
                        ui.label("Dukaten");
                        ui.label("Silbertaler");
                        ui.label("Heller");
                        ui.label("Kreuzer");
                        ui.end_row();
                        ui.add(egui::DragValue::new(dukaten));
                        ui.add(egui::DragValue::new(silber));
                        ui.add(egui::DragValue::new(heller));
                        ui.add(egui::DragValue::new(kreuzer));
                    });

                    // tier
                    {
                        let Animal {
                            name,
                            typus,
                            lo,
                            ap,
                            lep,
                            asp,
                            mu,
                            kl,
                            r#in,
                            ch,
                            ff,
                            ge,
                            ko,
                            kk,
                            sk,
                            zk,
                            rs,
                            ini,
                            gs,
                            attack,
                            at,
                            vw,
                            tp,
                            rw,
                            actions,
                            abilities,
                        } = animal;

                        egui::Grid::new("animal-strings-grid").show(ui, |ui| {
                            ui.label("Name");
                            text_edit!(ui, name, 200.0);
                            ui.end_row();
                            ui.label("Typus");
                            text_edit!(ui, typus, 200.0);
                            ui.end_row();
                        });
                        egui::Grid::new("animal-values-grid").show(ui, |ui| {
                            ui.label("LO");
                            ui.add(egui::DragValue::new(lo));
                            ui.label("AP");
                            ui.add(egui::DragValue::new(ap));
                            ui.label("LeP");
                            ui.add(egui::DragValue::new(lep));
                            ui.label("AsP");
                            ui.add(egui::DragValue::new(asp));
                            ui.end_row();
                            ui.label("MU");
                            ui.add(egui::DragValue::new(mu));
                            ui.label("KL");
                            ui.add(egui::DragValue::new(kl));
                            ui.label("IN");
                            ui.add(egui::DragValue::new(r#in));
                            ui.label("CH");
                            ui.add(egui::DragValue::new(ch));
                            ui.end_row();
                            ui.label("FF");
                            ui.add(egui::DragValue::new(ff));
                            ui.label("GE");
                            ui.add(egui::DragValue::new(ge));
                            ui.label("KO");
                            ui.add(egui::DragValue::new(ko));
                            ui.label("KK");
                            ui.add(egui::DragValue::new(kk));
                            ui.end_row();
                            ui.label("SK");
                            ui.add(egui::DragValue::new(sk));
                            ui.label("ZK");
                            ui.add(egui::DragValue::new(zk));
                            ui.label("RS");
                            ui.add(egui::DragValue::new(rs));
                            ui.label("INI");
                            ui.add(egui::DragValue::new(ini));
                            ui.end_row();
                            ui.label("GS");
                            ui.add(egui::DragValue::new(gs));
                            ui.label("Angriff");
                            ui.add(egui::DragValue::new(attack));
                            ui.label("AT");
                            ui.add(egui::DragValue::new(at));
                            ui.label("VW");
                            ui.add(egui::DragValue::new(vw));
                            ui.end_row();
                            ui.label("TP");
                            ui.add(egui::DragValue::new(tp));
                            ui.label("RW");
                            ui.add(egui::DragValue::new(rw));
                            ui.end_row();
                        });
                        egui::Grid::new("animal-vectors-grid").show(ui, |ui| {
                            ui.label("Aktionen");
                            ui.vertical(|ui| {
                                actions.retain_mut(|a| {
                                    let mut keep_action = true;
                                    ui.horizontal(|ui| {
                                        text_edit!(ui, a);
                                        if ui.button("x").clicked() {
                                            keep_action = false;
                                        }
                                    });
                                    keep_action
                                });
                            });
                            ui.end_row();
                            if ui.button("+").clicked() {
                                actions.push(String::new());
                            }
                            ui.end_row();

                            ui.label("Sonderfertigkeiten");
                            ui.vertical(|ui| {
                                abilities.retain_mut(|a| {
                                    let mut keep_ability = true;
                                    ui.horizontal(|ui| {
                                        text_edit!(ui, a);
                                        if ui.button("x").clicked() {
                                            keep_ability = false;
                                        }
                                    });
                                    keep_ability
                                });
                            });
                            ui.end_row();
                            if ui.button("+").clicked() {
                                abilities.push(String::new());
                            }
                            ui.end_row();
                        });
                    }
                });
        });*/
    }
}
