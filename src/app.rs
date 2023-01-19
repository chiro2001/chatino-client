use crate::chatino::{Action, Chatino, State};
use crate::emote::{emote_value, EMOTES};
use crate::message::ChatMessage;
use crate::ui::password::password;
use crate::user::User;
use eframe::emath::Align;
use egui::{Layout, RichText, Ui};
use std::ops::Add;
use std::time::{Duration, SystemTime};

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
                                for m in &self.messages {
                                    ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                                        ui.label(&m.extra);
                                        ui.label(&m.nick);
                                    });
                                    ui.label(&m.msg);
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
                            ui.label("聊天室");
                            ui.text_edit_singleline(&mut self.room);
                            ui.end_row();
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
                            // self.state = State::Chatting;
                            emmit_login = true;
                        }
                    });
                });
            });
            if emmit_login {
                // self.client = Some(ChatinoClient::new())
                match &self.action_tx {
                    None => {}
                    Some(tx) => {
                        tx.send(Action::Login(
                            self.room.to_string(),
                            self.me.nick.to_string(),
                            self.password.to_string(),
                        ))
                        .unwrap();
                    }
                }
            }
        }

        // parse recv data
        match &self.action_rx {
            None => {}
            Some(rx) => match rx.try_recv() {
                Ok(action) => match action {
                    Action::GetInfo => {}
                    Action::OnlineSet(v) => {
                        self.me.extra = v.trip;
                        self.users = v.nicks;
                        if self.state == State::NowLogin {
                            self.state = State::Chatting;
                        }
                    }
                    Action::OnlineRemove(v) => {
                        self.users = self
                            .users
                            .iter()
                            .filter(|x| x.to_string() != v.nick)
                            .map(|x| x.to_string())
                            .collect();
                    }
                    Action::OnlineAdd(v) => {
                        self.users.push(v.nick);
                    }
                    Action::Info(v) => {
                        self.messages.push(ChatMessage {
                            nick: "*".to_string(),
                            extra: v.trip,
                            time: SystemTime::UNIX_EPOCH.add(Duration::from_secs(v.time)),
                            msg: v.text,
                        });
                    }
                    Action::ChatNormal(v) => {
                        self.messages.push(ChatMessage {
                            nick: v.nick,
                            extra: v.trip,
                            time: SystemTime::UNIX_EPOCH.add(Duration::from_secs(v.time)),
                            msg: v.text,
                        });
                    }
                    Action::ChatWhisper(v) => {
                        self.messages.push(ChatMessage {
                            nick: v.nick,
                            extra: v.trip,
                            time: SystemTime::UNIX_EPOCH.add(Duration::from_secs(v.time)),
                            msg: v.text,
                        });
                    }
                    Action::RaiseError(_) => {}
                    _ => {}
                },
                Err(_) => {}
            },
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
                        ui.label(user);
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
