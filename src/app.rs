use crate::client::{ChatinoClient, CLIENT_KEY, CLIENT_NAME};
use crate::emote::{emote_value, EMOTES};
use crate::message::{
    get_chat_type, get_cmd, ChatMessage, CmdChatNormal, CmdChatWhisper, CmdGetInfoReq, CmdInfo,
    CmdJoinReq, CmdOnlineAdd, CmdOnlineRemove, CmdOnlineSet,
};
use crate::ui::password::password;
use crate::user::User;
use anyhow::Result;
use eframe::emath::Align;
use egui::{FontData, FontDefinitions, FontFamily, Layout, RichText, Ui};
use futures_util::{future, pin_mut, StreamExt};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum State {
    #[default]
    Index,
    Login,
    NowLogin,
    Chatting,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    GetInfo,
    OnlineSet(CmdOnlineSet),
    OnlineRemove(CmdOnlineRemove),
    OnlineAdd(CmdOnlineAdd),
    Info(CmdInfo),
    ChatNormal(CmdChatNormal),
    ChatWhisper(CmdChatWhisper),
    Login(String, String, String),
    SendMessage(ChatMessage),
    RecvMessage(ChatMessage),
    RaiseError(String),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChatSettings {
    pub sidebar_always_on: bool,
    pub sidebar_minimal: bool,
    pub notification: bool,
    pub show_user_enter_exit: bool,
    pub enable_code_highlight: bool,
    pub enable_image: bool,
    pub editor_single_line: bool,
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            sidebar_always_on: true,
            sidebar_minimal: false,
            notification: true,
            show_user_enter_exit: true,
            enable_code_highlight: true,
            enable_image: true,
            editor_single_line: true,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Chatino {
    pub room: String,
    pub me: User,
    #[serde(skip)]
    pub state: State,
    pub password: String,
    #[serde(skip)]
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub emote_key: String,
    pub settings: ChatSettings,
    #[serde(skip)]
    pub users: Vec<User>,
    #[serde(skip)]
    pub action_tx: Option<Sender<Action>>,
    #[serde(skip)]
    pub action_rx: Option<Receiver<Action>>,
}

impl Default for Chatino {
    fn default() -> Self {
        Self {
            room: "".to_string(),
            state: State::default(),
            password: "".to_owned(),
            messages: vec![],
            input: "".to_string(),
            emote_key: EMOTES.first().unwrap().0.to_string(),
            settings: Default::default(),
            users: vec![],
            me: Default::default(),
            // client: None,
            action_tx: None,
            action_rx: None,
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

        // start new thread with one message channel
        let (action_ui_tx, action_run_rx) = mpsc::channel();
        let (action_run_tx, action_ui_rx) = mpsc::channel();
        tokio::spawn(async {
            Chatino::background(action_run_tx, action_run_rx).await.unwrap();
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            Self {
                action_tx: Some(action_ui_tx),
                action_rx: Some(action_ui_rx),
                ..eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
            }
        } else {
            Self {
                action_tx: Some(action_ui_tx),
                action_rx: Some(action_ui_rx),
                ..Default::default()
            }
        }
    }

    pub async fn background(tx: Sender<Action>, rx: Receiver<Action>) -> Result<()> {
        let tx = Arc::new(Mutex::new(tx));
        // connect to server
        let client = ChatinoClient::new().await?;
        let client_available = Arc::new(Mutex::new(true));
        // let stop = Arc::new(Mutex::new(false));
        let (ws_send_tx, ws_send_rx) = futures_channel::mpsc::unbounded();
        // let stop_send = stop.clone();
        let client_to_ws = ws_send_rx.map(Ok).forward(client.writer);
        let ws_to_ui = {
            client.reader.for_each(|message| async {
                if let Ok(message) = message {
                    if *client_available.lock().await {
                        match message {
                            Message::Text(text) => match get_cmd(&text) {
                                None => {}
                                Some(cmd) => match cmd.as_str() {
                                    "onlineSet" => {
                                        let v: CmdOnlineSet = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineSet(v)).unwrap();
                                    }
                                    "onlineRemove" => {
                                        let v: CmdOnlineRemove =
                                            serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineRemove(v)).unwrap();
                                    }
                                    "onlineAdd" => {
                                        let v: CmdOnlineAdd = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineAdd(v)).unwrap();
                                    }
                                    "info" => {
                                        let v: CmdInfo = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::Info(v)).unwrap();
                                    }
                                    "chat" => match get_chat_type(&text) {
                                        None => {}
                                        Some(type_name) => match type_name.as_str() {
                                            "chat" => {
                                                let v: CmdChatNormal =
                                                    serde_json::from_str(&text).unwrap();
                                                tx.lock()
                                                    .await
                                                    .send(Action::ChatNormal(v))
                                                    .unwrap();
                                            }
                                            "whisper" => {
                                                let v: CmdChatWhisper =
                                                    serde_json::from_str(&text).unwrap();
                                                tx.lock()
                                                    .await
                                                    .send(Action::ChatWhisper(v))
                                                    .unwrap();
                                            }
                                            _ => {}
                                        },
                                    },
                                    _ => {
                                        tx.lock()
                                            .await
                                            .send(Action::RaiseError(format!(
                                                "Unimplemented command: {}",
                                                cmd
                                            )))
                                            .unwrap();
                                    }
                                },
                            },
                            _ => {}
                        }
                    }
                }
            })
        };
        pin_mut!(client_to_ws, ws_to_ui);
        future::select(ws_to_ui, client_to_ws).await;
        loop {
            // let client_available = client_available.clone();
            match rx.try_recv() {
                Ok(action) => match action {
                    Action::GetInfo => {
                        ws_send_tx
                            .unbounded_send(Message::Text(
                                serde_json::to_string(&CmdGetInfoReq::default()).unwrap(),
                            ))
                            .unwrap();
                        // *client_available.lock().await = false;
                    }
                    Action::Login(channel, nick, password) => {
                        ws_send_tx
                            .unbounded_send(Message::Text(
                                serde_json::to_string(&CmdJoinReq {
                                    cmd: "join".to_string(),
                                    channel,
                                    nick,
                                    password,
                                    clientName: CLIENT_NAME.to_string(),
                                    clientKey: CLIENT_KEY.to_string(),
                                })
                                .unwrap(),
                            ))
                            .unwrap();
                    }
                    Action::SendMessage(_) => {}
                    _ => {}
                },
                Err(_) => {}
            }
            sleep(Duration::from_millis(10)).await;
        }
    }
}

impl eframe::App for Chatino {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.settings.sidebar_minimal {
            if self.settings.sidebar_always_on {
                egui::SidePanel::right("side_panel").show(ctx, |ui| {
                    self.sidebar(ui);
                });
            } else {
                egui::Window::new("💠").show(ctx, |ui| {
                    self.sidebar(ui);
                });
            }
        }

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.add_enabled_ui(
                    self.state == State::Index || self.state == State::Chatting,
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
                                if self.settings.editor_single_line {
                                    ui.text_edit_singleline(&mut self.input);
                                } else {
                                    ui.text_edit_multiline(&mut self.input);
                                }
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
                    },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_enabled_ui(
                self.state == State::Index || self.state == State::Chatting,
                |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                        ui.heading(if self.room.is_empty() {
                            "主页"
                        } else {
                            &self.room
                        });
                        if self.settings.sidebar_minimal {
                            if ui.button("💠").clicked() {
                                self.settings.sidebar_minimal = false;
                            }
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.with_layout(
                            Layout::top_down(Align::LEFT).with_cross_justify(true),
                            |ui| {
                                for i in 0..100 {
                                    // let _ = ui.button("test");
                                    ui.label(format!("test no. {}", i));
                                }
                            },
                        );
                    });
                },
            );
        });

        if self.state == State::Login {
            let mut emmit_login = false;
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
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        if ui.button("登录").clicked() {
                            self.state = State::NowLogin;
                            emmit_login = true;
                        }
                    });
                });
            });
            if emmit_login {
                // self.client = Some(ChatinoClient::new())
            }
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl Chatino {
    fn sidebar(&mut self, ui: &mut Ui) {
        ui.add_enabled_ui(
            self.state == State::Index || self.state == State::Chatting,
            |ui| {
                if !self.settings.sidebar_minimal {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        if ui.button("最小化侧边栏").clicked() {
                            self.settings.sidebar_minimal = true;
                        }
                    });
                }
                {
                    let mut debug_on_hover = ui.ctx().debug_on_hover();
                    ui.checkbox(&mut debug_on_hover, "🐛 调试模式")
                        .on_hover_text("Show structure of the ui when you hover with the mouse");
                    ui.ctx().set_debug_on_hover(debug_on_hover);
                }
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
                ui.checkbox(&mut self.settings.editor_single_line, "单行编辑回车发送");

                if ui.button("清除数据").clicked() {
                    self.state = Default::default();
                    *ui.ctx().memory() = Default::default();
                    ui.close_menu();
                }
            },
        );
    }
}
