use hashbrown::HashSet;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub token: String,
    pub admins: HashSet<u64>,
    #[serde(rename = "blocked-users")]
    pub blocked_users: HashSet<u64>,
    #[serde(rename = "disabled-commands")]
    pub disabled_commands: HashSet<String>,
}
