use crate::emote::{emote_value, EMOTES};
use crate::message::Message;
use crate::ui::password::password;
use crate::user::User;
use eframe::emath::Align;
use egui::{FontData, FontDefinitions, FontFamily, Layout, RichText};

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum State {
    // #[default]
    Login,
    #[default]
    Chatting,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChatSettings {
    pub sidebar_always_on: bool,
    pub notification: bool,
    pub show_user_enter_exit: bool,
    pub enable_code_highlight: bool,
    pub enable_image: bool,
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            sidebar_always_on: false,
            notification: true,
            show_user_enter_exit: true,
            enable_code_highlight: true,
            enable_image: true,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Chatino {
    pub room: String,
    pub me: User,
    pub state: State,
    pub password: String,
    #[serde(skip)]
    pub messages: Vec<Message>,
    pub input: String,
    pub emote_key: String,
    pub settings: ChatSettings,
    #[serde(skip)]
    pub users: Vec<User>,
}

impl Default for Chatino {
    fn default() -> Self {
        Self {
            room: "公共聊天室".to_string(),
            state: State::default(),
            password: "".to_owned(),
            messages: vec![],
            input: "".to_string(),
            emote_key: EMOTES.first().unwrap().0.to_string(),
            settings: Default::default(),
            users: vec![],
            me: Default::default(),
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
                ui.heading("十字街");
                ui.label("一个简洁轻小的聊天网站");
                egui::warn_if_debug_build(ui);
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.label("邮箱：");
                    ui.hyperlink("mailto:mail@to.henrize.kim");
                });
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.label("切换主题：");
                    egui::widgets::global_dark_light_mode_switch(ui);
                });
                ui.separator();
                ui.label("帐号管理");
                ui.label(RichText::new("已登录的帐号：").small());
                ui.label(RichText::new(self.me.to_string()).small());
                if ui.button("清除帐号信息").clicked() {
                    self.me = User::default();
                    self.state = State::Login;
                }
                ui.collapsing("在线的用户", |ui| {
                    self.users.iter().for_each(|user| {
                        ui.label(&user.nick);
                    });
                });
                ui.separator();
                ui.label("设置");
                ui.checkbox(&mut self.settings.sidebar_always_on, "侧边栏常开");
                ui.checkbox(&mut self.settings.notification, "接收消息通知");
                ui.checkbox(&mut self.settings.show_user_enter_exit, "用户加入/退出提醒");
                ui.checkbox(&mut self.settings.enable_code_highlight, "启用代码高亮");
                ui.checkbox(&mut self.settings.enable_image, "查看图片消息");

                if ui.button("清除数据").clicked() {
                    self.state = Default::default();
                    *ui.ctx().memory() = Default::default();
                    ui.close_menu();
                }
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
                        .selected_text(&self.emote_key)
                        .show_ui(ui, |ui| {
                            EMOTES.iter().for_each(|(k, _v)| {
                                if ui
                                    .selectable_value(
                                        &mut self.emote_key,
                                        k.to_string(),
                                        k.to_string(),
                                    )
                                    .changed()
                                {
                                    self.input += emote_value(&self.emote_key);
                                };
                            });
                        });
                    if ui.button(&self.emote_key).clicked() {
                        self.input += emote_value(&self.emote_key);
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
                            ui.text_edit_singleline(&mut self.me.nick);
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
