use egui::{Button, FontData, FontDefinitions, FontFamily};
use crate::ui::password::{password, password_ui};

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum State {
    #[default]
    Login,
    Chatting,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Chatino {
    room: String,
    nick: String,
    state: State,
    #[serde(skip)]
    password: String,
}

impl Default for Chatino {
    fn default() -> Self {
        Self {
            room: "".to_string(),
            nick: "test".to_owned(),
            state: State::default(),
            password: "".to_owned(),
        }
    }
}

impl Chatino {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = FontDefinitions::default();
        let font_name = "ali";
        fonts.font_data.insert(font_name.to_owned(), FontData::from_static(include_bytes!("../assets/Ali_Puhui_Medium.ttf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, font_name.to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push(font_name.to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Chatino {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_enabled_ui(self.state == State::Chatting, |ui| {
                ui.heading("侧边栏");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_enabled_ui(self.state == State::Chatting, |ui| {
                ui.heading("eframe template");
                ui.hyperlink("https://github.com/emilk/chatino");
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/chatino/blob/master/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });
        });

        if self.state == State::Login {
            egui::Window::new("请登录").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("昵称");
                            ui.text_edit_singleline(&mut self.nick);
                            ui.end_row();
                            ui.label("密码(可留空)");
                            ui.add(password(&mut self.password));
                            ui.end_row();
                        });
                    ui.separator();
                    ui.add(Button::new("登录"))
                });
            });
        }
    }
}
