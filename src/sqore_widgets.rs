use eframe::{
    egui::{collapsing_header::CollapsingState, RichText, Widget},
    epaint::FontId,
};
use uuid::Uuid;
pub struct DialogNode {
    code: String,
    uuid: Uuid,
}

impl DialogNode {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            uuid: Uuid::now_v7(),
        }
    }
}

impl Widget for &mut DialogNode {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let id = ui.make_persistent_id(self.uuid);
        let ret = CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.heading(RichText::new("Dialog Node").font(FontId::proportional(42.0)))
            })
            .body(|ui| {
                ui.vertical(|ui| {
                    ui.label("This is a label");
                    ui.code_editor(&mut self.code);
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("delete").clicked() {
                            println!("Delete");
                        }
                        if ui.button("clone").clicked() {
                            println!("Clone");
                        }
                        if ui.button("destroy the sun").clicked() {
                            println!("Destroy the sun UwU");
                        }
                    })
                });
            });
        ret.0
    }
}
