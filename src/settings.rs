use config::{Config, Environment, File};
use hashbrown::HashSet;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Twitter {
    pub consumer_api_key: String,
    pub consumer_api_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub debug: bool,
    pub token: String,
    pub admins: HashSet<u64>,
    pub blocked_users: HashSet<u64>,
    pub disabled_commands: HashSet<String>,
    pub twitter: Twitter,
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        let mut config = Config::default();
        config
            .merge(File::with_name("settings"))
            .expect("Settings loaded from file")
            .merge(Environment::with_prefix("DOLAN"))
            .expect("Settings loaded from ENV");
        config.try_into().expect("Settings deserialization")
    };
}
