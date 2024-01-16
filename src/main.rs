use eframe::{
    egui::{self, ViewportBuilder},
    epaint::Vec2,
    NativeOptions,
};
use rfd::FileDialog;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([320.0, 640.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "SCore Dialog Builder",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<SCoreDialog>::default()
        }),
    )
}

#[derive(Default)]
struct SCoreDialog {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for SCoreDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::containers::TopBottomPanel::top("heading_panel").show(ctx, |ui| {
        //     ui.heading("SCore Dialog Builder");

        //     // let menu_icon = egui::Image::new(egui::include_image!("menu.svg"));
        //     // ui.menu_image_button(menu_icon, |ui| {
        //     //     if ui.button("Test Button").clicked() {
        //     //         println!("Hello there!");
        //     //     }
        //     //     if ui.button("Close").clicked() {
        //     //         ui.close_menu();
        //     //     }
        //     // });
        // });
        egui::TopBottomPanel::top("sqore-top")
            .default_height(32f32)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let menu_icon = egui::Image::new(egui::include_image!("menu.svg"))
                        .max_size(Vec2 { x: 32f32, y: 32f32 });
                    ui.menu_image_button(menu_icon, |ui| {
                        ui.menu_button("Sub Menu", |ui| {
                            if ui.button("Click Me!").clicked() {
                                print!("Hello mouse!");
                            }
                        });
                    });
                    ui.heading("Sqore Dialog Builder");
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Heyo!");
                if ui.button("Pick File").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
            });
            if let Some(path) = &self.picked_path {
                ui.horizontal_centered(|ui| {
                    ui.label("Path: ");
                    ui.monospace(path);
                });
            }
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Files");
                    for file in &self.dropped_files {
                        ui.label(&file.name);
                    }
                });
            }
        });
    }
}
