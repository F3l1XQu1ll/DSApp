use crate::data::Erfahrungsgrad;

macro_rules! erfahrungsgrad {
    ($n: expr, $ap: expr, $em: expr, $fm: expr, $km: expr, $epm: expr, $zm: expr, $fz: expr) => {
        Erfahrungsgrad {
            name: $n,
            ap_konto: $ap,
            eigenschaft_max: $em,
            fertigkeit_max: $fm,
            kampftechnik_max: $km,
            eingenschaftspunkte_max: $epm,
            zauber_max: $zm,
            fremdzauber: $fz,
        }
    };
}

pub const UNERFAHREN: Erfahrungsgrad = erfahrungsgrad!("Unerfahren", 900, 12, 10, 8, 95, 8, 0);

pub const DURCHSCHNITTLICH: Erfahrungsgrad =
    erfahrungsgrad!("Durchschnittlich", 1000, 13, 10, 10, 98, 10, 1);

pub const ERFAHREN: Erfahrungsgrad = erfahrungsgrad!("Erfahren", 1100, 14, 10, 12, 100, 12, 2);

pub const KOMPETENT: Erfahrungsgrad = erfahrungsgrad!("Kompetent", 1200, 15, 13, 14, 102, 14, 3);

pub const MEISTERLICH: Erfahrungsgrad =
    erfahrungsgrad!("Meisterlich", 1400, 16, 16, 16, 105, 16, 4);

pub const BRILLIANT: Erfahrungsgrad = erfahrungsgrad!("Brillant", 1700, 17, 19, 18, 109, 18, 5);

pub const LEGENDAER: Erfahrungsgrad = erfahrungsgrad!("Legendär", 2100, 18, 20, 20, 114, 20, 6);

#[derive(Default, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum Erfahrungsgrade {
    UNERFAHREN,
    DURCHSCHNITTLICH,
    ERFAHREN,
    #[default]
    KOMPETENT,
    MEISTERLICH,
    BRILLIANT,
    LEGENDAER,
}

impl Erfahrungsgrade {
    pub fn erfahrungsgrad(&self) -> &'static Erfahrungsgrad {
        match *self {
            Erfahrungsgrade::UNERFAHREN => &UNERFAHREN,
            Erfahrungsgrade::DURCHSCHNITTLICH => &DURCHSCHNITTLICH,
            Erfahrungsgrade::ERFAHREN => &ERFAHREN,
            Erfahrungsgrade::KOMPETENT => &KOMPETENT,
            Erfahrungsgrade::MEISTERLICH => &MEISTERLICH,
            Erfahrungsgrade::BRILLIANT => &BRILLIANT,
            Erfahrungsgrade::LEGENDAER => &LEGENDAER,
        }
    }
}

impl crate::display::BuildUi for Erfahrungsgrade {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_source("erfahrungsgrad")
            .selected_text(self.erfahrungsgrad().name)
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Self::UNERFAHREN, UNERFAHREN.name);
                ui.selectable_value(self, Self::DURCHSCHNITTLICH, DURCHSCHNITTLICH.name);
                ui.selectable_value(self, Self::ERFAHREN, ERFAHREN.name);
                ui.selectable_value(self, Self::KOMPETENT, KOMPETENT.name);
                ui.selectable_value(self, Self::MEISTERLICH, MEISTERLICH.name);
                ui.selectable_value(self, Self::LEGENDAER, LEGENDAER.name);
            });
        crate::widgets::infobutton::Infobutton::new().show_ui(ui, |ui| {
            egui::Grid::new("erfahrungskonto-info")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    ui.strong("AP-Konto");
                    ui.label(format!("{}", self.erfahrungsgrad().ap_konto));
                    ui.end_row();

                    ui.strong("Höchstwert\nEigenschaft");
                    ui.label(format!("{}", self.erfahrungsgrad().eigenschaft_max));
                    ui.end_row();

                    ui.strong("Höchstwert\nFertigkeit");
                    ui.label(format!("{}", self.erfahrungsgrad().fertigkeit_max));
                    ui.end_row();

                    ui.strong("Höchstwert\nKampftechnik");
                    ui.label(format!("{}", self.erfahrungsgrad().kampftechnik_max));
                    ui.end_row();

                    ui.strong("maximale\nEigenschaftspunkte");
                    ui.label(format!("{}", self.erfahrungsgrad().eingenschaftspunkte_max));
                    ui.end_row();

                    ui.strong("maximale\nAnzahl der\nZauber /\nLitrgien");
                    ui.label(format!("{}", self.erfahrungsgrad().zauber_max));
                    ui.end_row();

                    ui.strong("(davon\nFremdzauber)");
                    ui.label(format!("{}", self.erfahrungsgrad().fremdzauber));
                    ui.end_row();
                });
        });
    }
}
