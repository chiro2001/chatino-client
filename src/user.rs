use std::fmt::{Display, Formatter};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct User {
    pub nick: String,
    pub extra: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.extra, self.nick)
    }
}