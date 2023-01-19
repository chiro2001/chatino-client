use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Message {
    pub nick: String,
    pub extra: String,
    pub time: SystemTime,
    pub msg: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            nick: "<INVALID>".to_string(),
            extra: "".to_string(),
            time: UNIX_EPOCH,
            msg: "<INVALID MESSAGE>".to_string(),
        }
    }
}