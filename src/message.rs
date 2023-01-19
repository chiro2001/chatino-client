#![allow(dead_code)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct CmdType {
    cmd: String,
}

#[derive(Serialize)]
pub struct CmdJoin {
    cmd: String,
    channel: String,
    password: String,
    clientName: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdOnlineSet {
    cmd: String,
    nicks: Vec<String>,
    trip: String,
    key: String,
    ver: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdOnlineRemove {
    cmd: String,
    nick: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdOnlineAdd {
    cmd: String,
    nick: String,
    trip: String,
    utype: String,
    level: u32,
    client: String,
    channel: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdInfo {
    cmd: String,
    trip: String,
    text: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdChatType {
    cmd: String,
    #[serde(rename = "type")]
    type_name: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdChatWhisper {
    cmd: String,
    #[serde(rename = "type")]
    type_name: String,
    from: String,
    level: u32,
    uType: String,
    nick: String,
    trip: String,
    text: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdChatNormal {
    cmd: String,
    #[serde(rename = "type")]
    type_name: String,
    nick: String,
    text: String,
    level: u32,
    member: bool,
    admin: bool,
    trip: String,
    time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CmdChatReq {
    cmd: String,
    text: String,
}

pub fn get_cmd(source: &str) -> Option<String> {
    let t: CmdType = serde_json::from_str(source).unwrap();
    match t.cmd {
        cmd if cmd.is_empty() => None,
        cmd => Some(cmd),
    }
}

pub fn get_chat_type(source: &str) -> Option<String> {
    let t: CmdChatType = serde_json::from_str(source).unwrap();
    match t.type_name {
        empty if empty.is_empty() => None,
        type_name => Some(type_name),
    }
}

#[cfg(test)]
mod test {
    use crate::message::{CmdOnlineSet, CmdType};

    #[test]
    fn test_serialization() -> anyhow::Result<()> {
        let text = r#"{
            "cmd": "test",
            "data": "extra"
        }"#;
        let t: CmdType = serde_json::from_str(&text).unwrap();
        println!("cmd type: {}", t.cmd);
        Ok(())
    }
}
