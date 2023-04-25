use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub debug: bool,
    pub token: String,
    pub openai: String,
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        let config = Config::builder()
            .add_source(File::with_name("settings"))
            .add_source(Environment::with_prefix("DOLAN"));
        config
            .build()
            .expect("Config to build")
            .try_deserialize()
            .expect("Settings deserialization")
    };
}
