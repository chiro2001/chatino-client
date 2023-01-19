use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref EMOTES: HashMap<&'static str, &'static str> =
        include!("../assets/emotes.txt").iter().copied().collect();
}
