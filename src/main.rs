#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]

mod commands;
mod settings;
mod utils;

use crate::settings::SETTINGS;
use commands::{omega::*, ping::*, repl::*, russia::*, time::*, trump::*, weather::*};
use log::{debug, info, LevelFilter};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::gateway::{Activity, Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// Discord Event Handler
struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        let activity = Activity::playing("DuckTales");
        context.set_activity(activity);
        info!("{} is connected...", ready.user.name);
    }
}

// Command Groups
group!({
    name: "general",
    options: {},
    commands: [ping, omega, time, repl, russia, trump, weather]
});

fn main() {
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

    // Initialize connection to Discord via token
    let mut client = Client::new(settings.token.as_str(), Handler).expect("Connection to Discord");
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Get owner information
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Build the framework setup
    let framework = StandardFramework::new()
        .configure(|c| {
            c.allow_dm(true)
                .case_insensitivity(true)
                .no_dm_prefix(true)
                .on_mention(Some(bot_id))
                .prefix("?")
                .owners(owners)
                .with_whitespace(false)
        })
        .before(|_ctx, msg, command| {
            debug!(
                "Received command '{}' from '{}#{}'",
                command, msg.author.name, msg.author.discriminator
            );
            true
        })
        .group(&GENERAL_GROUP);
    client.with_framework(framework);

    // Start the application
    if let Err(e) = client.start_autosharded() {
        println!("Uh-oh, Dolan malfunctioned: {:#?}", e);
    }
}
