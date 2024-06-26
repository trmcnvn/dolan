#![feature(lazy_cell)]

mod commands;
mod settings;
mod utils;

use self::commands::{
    coder::CODER_COMMAND, gpt::GPT_COMMAND, llama::LLAMA_COMMAND, ping::PING_COMMAND,
    repl::REPL_COMMAND, sdiff::SDIFF_COMMAND, time::TIME_COMMAND, translate::TRANSLATE_COMMAND,
    weather::WEATHER_COMMAND,
};
use crate::settings::SETTINGS;
use axum::{routing::get, Router};
use log::{info, LevelFilter};
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{group, hook},
            CommandError, Configuration,
        },
        StandardFramework,
    },
    gateway::{ActivityData, ShardManager},
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::spawn;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// Discord Event Handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        let activity = ActivityData::playing("DuckTales");
        context.set_activity(Some(activity));
        info!("{} is connected...", ready.user.name);
    }
}

// Command Groups
#[group]
#[commands(ping, time, repl, weather, gpt, llama, coder, translate, sdiff)]
struct General;

#[hook]
async fn before_hook(_: &Context, msg: &Message, cmd_name: &str) -> bool {
    log::info!("Received command {} from {}", cmd_name, msg.author.name);
    true
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(why) = error {
        log::error!("Error in {}: {:?}", cmd_name, why);
    }
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // Load config
    let settings = SETTINGS.clone();

    // Initialize logging
    if env::var_os("RUST_LOG").is_none() {
        let level = if settings.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };
        env::set_var("RUST_LOG", format!("dolan={}", level));
    }
    pretty_env_logger::init();

    // Start webserver for health checks
    let webapp = Router::new().route("/healthz", get(health_check));
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var_os("PORT")
            .unwrap_or("10000".into())
            .into_string()
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    ));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let web_await = spawn(async move {
        axum::serve(listener, webapp).await.unwrap();
    });

    // Build the framework setup
    let framework = StandardFramework::new()
        .before(before_hook)
        .group(&GENERAL_GROUP);
    framework.configure(
        Configuration::new()
            .allow_dm(true)
            .case_insensitivity(true)
            .no_dm_prefix(true)
            .prefix("?")
            .with_whitespace(false),
    );

    // Initialize connection to Discord via token
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(settings.token.as_str(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Create the Discord client");

    // Start the application
    let discord_await = spawn(async move {
        client.start().await.unwrap();
    });

    // Wait for the awaiters!
    if let Err(e) = tokio::try_join!(web_await, discord_await) {
        log::error!("Error: {:?}", e)
    };
}
