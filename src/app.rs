use crate::emote::{emote_value, EMOTES};
use crate::message::Message;
use crate::ui::password::password;
use eframe::emath::Align;
use egui::{FontData, FontDefinitions, FontFamily, Layout};

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum State {
    // #[default]
    Login,
    #[default]
    Chatting,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Chatino {
    pub room: String,
    pub nick: String,
    pub state: State,
    pub password: String,
    #[serde(skip)]
    pub messages: Vec<Message>,
    pub input: String,
    pub emote: String,
}

impl Default for Chatino {
    fn default() -> Self {
        Self {
            room: "公共聊天室".to_string(),
            nick: "test".to_owned(),
            state: State::default(),
            password: "".to_owned(),
            messages: vec![],
            input: "".to_string(),
            emote: EMOTES.first().unwrap().0.to_string(),
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
        fonts.font_data.insert(
            font_name.to_owned(),
            FontData::from_static(include_bytes!("../assets/Ali_Puhui_Medium.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, font_name.to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
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
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_enabled_ui(self.state == State::Chatting, |ui| {
                egui::warn_if_debug_build(ui);
                ui.heading("侧边栏");
                if ui.button("清除数据").clicked() {}
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
                        ui.text_edit_multiline(&mut self.input);
                    });
                });
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui.button("<发送>").clicked() {
                        self.input.clear();
                    }
                    egui::ComboBox::from_id_source("emote_select")
                        .selected_text(&self.emote)
                        .show_ui(ui, |ui| {
                            EMOTES.iter().for_each(|(k, _v)| {
                                if ui
                                    .selectable_value(
                                        &mut self.emote,
                                        k.to_string(),
                                        k.to_string(),
                                    )
                                    .changed()
                                {
                                    self.input += emote_value(&self.emote);
                                };
                            });
                        });
                    if ui.button(&self.emote).clicked() {
                        self.input += emote_value(&self.emote);
                    }
                    if ui.button("图片").clicked() {}
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_enabled_ui(self.state == State::Chatting, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&self.room);
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            for i in 0..100 {
                                // let _ = ui.button("test");
                                ui.label(format!("test no. {}", i));
                            }
                        },
                    );
                });
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
                    if ui.button("登录").clicked() {
                        self.state = State::Chatting
                    }
                });
            });
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
