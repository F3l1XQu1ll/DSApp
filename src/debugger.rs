pub struct Debugger<'a> {
    ctx: &'a egui::Context,
    app: &'a crate::DSApp,
    data_code: &'a mut String,
    autoupdate_code: &'a mut bool,
    replace_data: &'a mut bool,
}

impl<'a> Debugger<'a> {
    pub fn new(
        ctx: &'a egui::Context,
        app: &'a crate::app::DSApp,
        data_code: &'a mut String,
        autoupdate_code: &'a mut bool,
        replace_data: &'a mut bool,
    ) -> Self {
        Self {
            ctx,
            app,
            data_code,
            autoupdate_code,
            replace_data,
        }
    }

    pub fn show(&mut self) {
        egui::Window::new("Code")
            .constrain(true)
            .show(self.ctx, |ui| {
                let ron_code =
                    ron::ser::to_string_pretty(&self.app, ron::ser::PrettyConfig::default())
                        .unwrap();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let mut enable_debug = self.ctx.debug_on_hover();
                        ui.checkbox(&mut enable_debug, "Debug");
                        ui.checkbox(&mut self.autoupdate_code, "Auto Update");
                        self.ctx.set_debug_on_hover(enable_debug);
                        if ui.button("Reset").clicked() {
                            ui.ctx().memory_mut(|mem| *mem = Default::default());
                        }
                        if ui.button("Kopieren").clicked() {
                            self.ctx.output_mut(|o| o.copied_text = ron_code.clone());
                        }
                        if ui.button("Übernehmen").clicked() {
                            *self.replace_data = true;
                        }
                        if ui.button("Aktualisieren").clicked() || *self.autoupdate_code {
                            *self.data_code = ron_code;
                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_enabled(
                            true,
                            egui::TextEdit::multiline(self.data_code).code_editor(),
                        );
                    });
                });
            });
    }
}
