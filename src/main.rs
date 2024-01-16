use eframe::{
    egui::{self, Response, RichText, Ui, ViewportBuilder, Widget},
    epaint::{FontId, Vec2},
    CreationContext, NativeOptions,
};
use rfd::FileDialog;
use sqore_widgets::DialogNode;

mod sqore_widgets;

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
            Box::new(SCoreDialog::new(cc))
        }),
    )
}

#[derive(Default)]
struct SCoreDialog {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    dialog_nodes: Vec<DialogNode>,
    font_size: f32,
}

impl SCoreDialog {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            font_size: 32f32,
            ..Default::default()
        }
    }

    fn label(&self, ui: &mut Ui, text: &str) -> Response {
        ui.label(RichText::new(text).font(FontId::proportional(self.font_size)))
    }
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
                        if ui.button("Add Node").clicked() {
                            self.dialog_nodes
                                .push(DialogNode::new("what a fucking node"));
                        }
                    });
                    ui.heading("Sqore Dialog Builder");
                });
            });
        egui::SidePanel::left("options_panel").show(ctx, |ui| {
            self.label(ui, "text");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                self.label(ui, "Heyo!");
                if ui.button("Pick File").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
            });
            if let Some(path) = &self.picked_path {
                ui.horizontal_centered(|ui| {
                    self.label(ui, "Path: ");
                    ui.monospace(path);
                });
            }
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    self.label(ui, "Files");
                    for file in &self.dropped_files {
                        let name = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        self.label(ui, name.as_str());
                    }
                });
            }
            ui.heading("Nodes");
            for node in &mut self.dialog_nodes {
                node.ui(ui);
            }
        });
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files = i.raw.dropped_files.clone();
            }
        });
    }
}
