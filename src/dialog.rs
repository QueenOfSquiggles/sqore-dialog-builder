use eframe::{
    egui::{collapsing_header::CollapsingState, ComboBox, Response, Ui, Widget},
    epaint::Color32,
};
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default)]
pub enum Line {
    Text {
        text: String,
        character: String,
        requires: String,
    },
    Choice {
        prompt: String,
        character: String,
        options: Vec<ChoiceOptionEntry>,
    },
    Action {
        action: String,
    },
    Signal {
        name: String,
        args: Vec<String>,
    },
    #[default]
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct ChoiceOptionEntry {
    pub text: String,
    pub requires: String,
    pub action: String,
}

impl Line {
    pub fn default_text() -> Self {
        Self::Text {
            text: "".into(),
            character: "".into(),
            requires: "".into(),
        }
    }
    pub fn default_choice() -> Self {
        Self::Choice {
            prompt: "".into(),
            character: "".into(),
            options: Vec::new(),
        }
    }

    pub fn default_action() -> Self {
        Self::Action { action: "".into() }
    }

    pub fn default_signal() -> Self {
        Self::Signal {
            name: "".into(),
            args: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LineWidget {
    line: Line,
}
impl Widget for &mut LineWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered_justified(|ui| {
            let cached = self.get_type_code();
            let mut _type: &str = &self.get_type_code();
            ComboBox::from_label("Type")
                .selected_text(_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut _type, "text", "Text");
                    ui.selectable_value(&mut _type, "choice", "Choice");
                    ui.selectable_value(&mut _type, "action", "Action");
                    ui.selectable_value(&mut _type, "signal", "Signal");
                    ui.selectable_value(&mut _type, "none", "None");
                });
            if cached != _type {
                self.line = self.get_from_type_code(_type);
            }
            let n_line = LineWidget::render(ui, self.line.clone());
            self.line = n_line;
        })
        .response
    }
}

impl LineWidget {
    fn render(ui: &mut Ui, line: Line) -> Line {
        match line {
            Line::Text {
                text,
                character,
                requires,
            } => Self::render_text(ui, text, character, requires),
            Line::Choice {
                prompt,
                character,
                options,
            } => Self::render_choice(ui, prompt, character, options),
            Line::Action { action } => Self::render_action(ui, action),
            Line::Signal { name, args } => Self::render_signal(ui, name, args),
            Line::None => Self::render_none(ui),
        }
    }
    fn render_text(ui: &mut Ui, p_text: String, p_character: String, p_requires: String) -> Line {
        let mut text = p_text.clone();
        let mut character = p_character.clone();
        let mut requires = p_requires.clone();
        let lbl_char = ui.label("Character").id;
        ui.text_edit_singleline(&mut character)
            .labelled_by(lbl_char);
        let lbl_text = ui.label("Text").id;
        ui.text_edit_multiline(&mut text).labelled_by(lbl_text);
        let lbl_req = ui.label("Requirements").id;
        ui.text_edit_singleline(&mut requires).labelled_by(lbl_req);
        Line::Text {
            text,
            character,
            requires,
        }
    }
    fn render_choice(
        ui: &mut Ui,
        p_prompt: String,
        p_character: String,
        p_options: Vec<ChoiceOptionEntry>,
    ) -> Line {
        let mut prompt = p_prompt.clone();
        let mut character = p_character.clone();
        let mut options = p_options.clone();
        let lbl_char = ui.label("Character").id;
        ui.text_edit_singleline(&mut character)
            .labelled_by(lbl_char);
        let lbl_text = ui.label("Prompt").id;
        ui.text_edit_multiline(&mut prompt).labelled_by(lbl_text);
        for opt in options.iter_mut() {
            let id = ui.make_persistent_id(Uuid::now_v7());
            CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    ui.label(opt.text.clone());
                })
                .body(|ui| {
                    let lbl = ui.label("Option Text").id;
                    ui.text_edit_singleline(&mut opt.text).labelled_by(lbl);
                    let lbl = ui.label("Requirements").id;
                    ui.text_edit_singleline(&mut opt.requires).labelled_by(lbl);
                    let lbl = ui.label("Action").id;
                    ui.text_edit_singleline(&mut opt.action).labelled_by(lbl);
                });
        }
        ui.horizontal_centered(|ui| {
            if ui.button("Add").clicked() {
                options.push(ChoiceOptionEntry::default());
            }
        });
        Line::Choice {
            prompt,
            character,
            options,
        }
    }
    fn render_action(ui: &mut Ui, p_action: String) -> Line {
        let mut action = p_action.clone();
        let lbl = ui.label("Action Code").id;
        ui.text_edit_singleline(&mut action).labelled_by(lbl);
        Line::Action { action }
    }
    fn render_signal(ui: &mut Ui, p_name: String, p_args: Vec<String>) -> Line {
        let mut name = p_name.clone();
        let mut args = p_args.clone();
        let lbl = ui.label("Signal Name").id;
        ui.text_edit_singleline(&mut name).labelled_by(lbl);
        ui.label("Args:");
        let mut dead = Vec::new();
        for (index, arg) in args.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(arg);
                if ui.button("Delete").clicked() {
                    dead.push(index);
                }
            });
        }

        // remove all dead items "in-place" so we can go off of index when deleting more than one item (unlikely but possible)
        args = args
            .iter()
            .enumerate()
            .filter_map(|(idx, arg)| if dead.contains(&idx) { None } else { Some(arg) })
            .cloned()
            .collect();
        Line::Signal { name, args }
    }

    fn render_none(ui: &mut Ui) -> Line {
        ui.label("Empty node. Select something.");
        Line::None
    }

    pub fn get_name(&self) -> String {
        #[allow(unused_variables)]
        match self.line.clone() {
            Line::Text {
                text,
                character,
                requires,
            } => format!("{} : {}", character, Self::get_short_text(text)),
            Line::Choice {
                prompt,
                character,
                options,
            } => format!("Choice {} #{}", Self::get_short_text(prompt), options.len()),
            Line::Action { action } => format!("Action {}", Self::get_short_text(action)),
            Line::Signal { name, args } => format!("Signal {}", name),
            Line::None => "Empty Node".to_owned(),
        }
    }

    fn get_short_text(text: String) -> String {
        if text.is_empty() {
            return "".into();
        }
        const SHORT_LEN: usize = 8;
        let len = SHORT_LEN.min(text.len() - 1);
        text.split_at(len).0.to_owned()
    }

    pub fn get_color(&self) -> Color32 {
        #[allow(unused_variables)]
        match self.line.clone() {
            Line::Text {
                text,
                character,
                requires,
            } => Color32::GRAY,
            Line::Choice {
                prompt,
                character,
                options,
            } => Color32::LIGHT_BLUE,
            Line::Action { action } => Color32::LIGHT_GREEN,
            Line::Signal { name, args } => Color32::LIGHT_RED,
            Line::None => Color32::WHITE,
        }
    }

    fn get_type_code(&self) -> &str {
        #[allow(unused_variables)]
        match self.line.clone() {
            Line::Text {
                text,
                character,
                requires,
            } => "text",
            Line::Choice {
                prompt,
                character,
                options,
            } => "choice",
            Line::Action { action } => "action",
            Line::Signal { name, args } => "signal",
            Line::None => "none",
        }
    }

    fn get_from_type_code(&self, type_code: &str) -> Line {
        match type_code {
            "text" => Line::default_text(),
            "choice" => Line::default_choice(),
            "action" => Line::default_action(),
            "signal" => Line::default_signal(),
            _ => Line::None,
        }
    }
}
