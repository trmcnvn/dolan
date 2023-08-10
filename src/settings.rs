use config::{Config, Environment, File};
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub debug: bool,
    pub token: String,
    pub openai: String,
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    let config = Config::builder()
        .add_source(File::with_name("settings").required(false))
        .add_source(Environment::with_prefix("DOLAN"));
    config
        .build()
        .expect("Config to build")
        .try_deserialize()
        .expect("Settings deserialization")
});
