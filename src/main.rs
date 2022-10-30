mod commands;
mod settings;
mod utils;

use self::commands::{
    ping::PING_COMMAND, repl::REPL_COMMAND, time::TIME_COMMAND, weather::WEATHER_COMMAND,
};
use crate::settings::SETTINGS;
use axum::{routing::get, Router};
use log::{info, LevelFilter};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::{group, hook},
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
    },
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
        let activity = Activity::playing("DuckTales");
        context.set_activity(activity).await;
        info!("{} is connected...", ready.user.name);
    }
}

// Command Groups
#[group]
#[commands(ping, time, repl, weather)]
struct General;

#[hook]
async fn before_hook(_: &Context, msg: &Message, cmd_name: &str) -> bool {
    println!(
        "Recevied command {} from {}#{}",
        cmd_name, msg.author.name, msg.author.discriminator
    );
    true
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
    let addr = SocketAddr::from(([0, 0, 0, 0], 10000));
    let web_await = spawn(axum::Server::bind(&addr).serve(webapp.into_make_service()));

    // Build the framework setup
    let framework = StandardFramework::new()
        .configure(|c| {
            c.allow_dm(true)
                .case_insensitivity(true)
                .no_dm_prefix(true)
                .prefix("?")
                .with_whitespace(false)
        })
        .before(before_hook)
        .group(&GENERAL_GROUP);

    // Initialize connection to Discord via token
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(settings.token.as_str(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Create the Discord client");

    // Start the application
    let discord_await = spawn(async move {
        client.start_autosharded().await.unwrap();
    });

    // Wait for the awaiters!
    if let Err(e) = tokio::try_join!(web_await, discord_await) {
        println!("Error: {:?}", e)
    };
}
