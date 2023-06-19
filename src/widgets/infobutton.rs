pub struct Infobutton;

impl Infobutton {
    pub fn new() -> Self {
        Self
    }

    pub fn show_text(
        &self,
        ui: &mut egui::Ui,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response {
        self.make_button(ui).on_hover_text(text)
    }

    pub fn show_ui(
        &self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::Response {
        self.make_button(ui).on_hover_ui(add_contents)
    }

    fn make_button(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::Button::new("ℹ")
                .frame(false)
                .sense(egui::Sense::hover().union(egui::Sense::click())),
        )
    }
}
