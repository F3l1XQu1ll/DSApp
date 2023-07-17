#[macro_export]
macro_rules! drag_val {
    ($ui:expr, $label: expr, $link: expr) => {
        $ui.strong($label);
        drag_val!($ui, $link)
    };
    ($ui:expr, $label: expr, $link: expr, min: $min: literal) => {
        $ui.strong($label);
        drag_val!($ui, $link, min $min)
    };
    ($ui:expr, $link: expr) => {
        $ui.horizontal(|ui| {
            if ui
                .add_enabled(*$link > crate::math::MinMax::MIN, egui::Button::new("–"))
                .clicked()
            {
                *$link -= 1;
            }
            ui.add(egui::DragValue::new($link).clamp_range(0..=1000));
            if ui
                .add_enabled(*$link < crate::math::MinMax::MAX, egui::Button::new("+"))
                .clicked()
            {
                *$link += 1;
            }
        });
    };
    ($ui:expr, $link: expr, min $min: literal) => {
        $ui.horizontal(|ui| {
            if ui
                .add_enabled(*$link > $min, egui::Button::new("–"))
                .clicked()
            {
                *$link -= 1;
            }
            ui.add(egui::DragValue::new($link).clamp_range($min..=1000));
            if ui
                .add_enabled(*$link < crate::math::MinMax::MAX, egui::Button::new("+"))
                .clicked()
            {
                *$link += 1;
            }
        });
    };
}

#[macro_export]
macro_rules! text_edit {
    ($ui:expr, $link: expr) => {
        $ui.text_edit_singleline($link)
    };
    ($ui:expr, $link: expr, $width: expr) => {
        egui::TextEdit::singleline($link)
            .min_size(egui::vec2($width, 1.0))
            .show($ui)
    };
}

#[macro_export]
macro_rules! debug_ui {
    ($ui:expr, $text: expr) => {
        if cfg!(debug_assertions) {
            $ui.add(egui::Label::new(
                egui::RichText::new($text)
                    .color(egui::Color32::RED)
                    .monospace(),
            ));
        }
    };
}
