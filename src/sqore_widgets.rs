use eframe::{
    egui::{collapsing_header::CollapsingState, RichText, Widget},
    epaint::FontId,
};

use uuid::Uuid;

use crate::dialog::LineWidget;
pub struct DialogNode {
    uuid: Uuid,
    line: LineWidget,
}

impl DialogNode {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            line: Default::default(),
        }
    }
}

impl Widget for &mut DialogNode {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let id = ui.make_persistent_id(self.uuid);
        #[allow(unused_variables)]
        let heading = self.line.get_name();
        let color = self.line.get_color();
        let ret = CollapsingState::load_with_default_open(ui.ctx(), id, false)
            .show_header(ui, |ui| {
                ui.heading(
                    RichText::new(heading.clone())
                        .font(FontId::proportional(42.0))
                        .color(color),
                )
            })
            .body(|ui| {
                self.line.ui(ui);
            });
        ret.0
    }
}
