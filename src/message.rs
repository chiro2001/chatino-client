use std::time::{SystemTime, UNIX_EPOCH};
use serde::Deserialize;

#[derive(Debug)]
pub struct ChatMessage {
    pub nick: String,
    pub extra: String,
    pub time: SystemTime,
    pub msg: String,
}

impl Default for ChatMessage {
    fn default() -> Self {
        Self {
            nick: "<INVALID>".to_string(),
            extra: "".to_string(),
            time: UNIX_EPOCH,
            msg: "<INVALID MESSAGE>".to_string(),
        }
    }
}

#[allow(non_snake_case)]
pub struct CmdJoin {
    cmd: String,
    channel: String,
    password: String,
    clientName: String,
}

#[derive(Deserialize)]
pub struct CmdOnlineSetResp {
    cmd: String,
    nicks: Vec<String>,
    trip: String,
    key: String,
    ver: String,
    time: u64,
}
