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
    pub(crate) cmd: String,
}

#[derive(Serialize)]
pub struct CmdJoinReq {
    pub(crate) cmd: String,
    pub(crate) channel: String,
    pub(crate) nick: String,
    pub(crate) password: String,
    pub(crate) clientName: String,
    pub(crate) clientKey: String,
}

#[derive(Serialize)]
pub struct CmdGetInfoReq {
    pub(crate) cmd: String,
}
impl Default for CmdGetInfoReq {
    fn default() -> Self {
        Self {
            cmd: "getinfo".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdOnlineSet {
    pub(crate) cmd: String,
    pub(crate) nicks: Vec<String>,
    pub(crate) trip: String,
    pub(crate) key: String,
    pub(crate) ver: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdOnlineRemove {
    pub(crate) cmd: String,
    pub(crate) nick: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdOnlineAdd {
    pub(crate) cmd: String,
    pub(crate) nick: String,
    pub(crate) trip: String,
    pub(crate) utype: String,
    pub(crate) level: u32,
    pub(crate) client: String,
    pub(crate) channel: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdInfo {
    pub(crate) cmd: String,
    pub(crate) trip: String,
    pub(crate) text: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdChatType {
    pub(crate) cmd: String,
    #[serde(rename = "type")]
    pub(crate) type_name: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdChatWhisper {
    pub(crate) cmd: String,
    #[serde(rename = "type")]
    pub(crate) type_name: String,
    pub(crate) from: String,
    pub(crate) level: u32,
    pub(crate) uType: String,
    pub(crate) nick: String,
    pub(crate) trip: String,
    pub(crate) text: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdChatNormal {
    pub(crate) cmd: String,
    #[serde(rename = "type")]
    pub(crate) type_name: String,
    pub(crate) nick: String,
    pub(crate) text: String,
    pub(crate) level: u32,
    pub(crate) member: bool,
    pub(crate) admin: bool,
    pub(crate) trip: String,
    pub(crate) time: u64,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Default)]
#[serde(default)]
pub struct CmdChatReq {
    pub(crate) cmd: String,
    pub(crate) text: String,
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
