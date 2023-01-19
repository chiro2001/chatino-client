pub const EMOTES: &'static [(&'static str, &'static str)] = &include!("../assets/emotes.txt");

pub fn emote_value(key: &str) -> &'static str {
    EMOTES
        .iter()
        .find(|(k, _)| k.to_string() == key.to_string())
        .unwrap_or(&("", ""))
        .1
}
