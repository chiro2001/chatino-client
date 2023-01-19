use crate::background::background;
use crate::emote::EMOTES;
use crate::message::{
    get_cmd, ChatMessage, CmdChatNormal, CmdChatWhisper, CmdGetInfoReq, CmdInfo, CmdJoinReq,
    CmdOnlineAdd, CmdOnlineRemove, CmdOnlineSet,
};
use crate::user::User;
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum State {
    #[default]
    Index,
    Login,
    NowLogin,
    Chatting,
}

#[allow(dead_code)]
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
            background(action_run_tx, action_run_rx).await.unwrap();
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
}
