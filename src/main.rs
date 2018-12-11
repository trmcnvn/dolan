#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]

use crate::settings::SETTINGS;
use env_logger::{Builder, WriteStyle};
use log::{debug, LevelFilter};
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{help_commands, StandardFramework};
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;

mod commands;
mod settings;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, _ready: Ready) {
        context.set_game("?help");
        println!("Dolan is connected...");
    }
}

fn main() {
    // Load config
    let settings = SETTINGS.clone();

    // Initialize logging
    let mut builder = Builder::new();
    let level = if settings.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    builder
        .filter(None, level)
        .write_style(WriteStyle::Always)
        .init();

    // Initialize connection to Discord via token
    let mut client = Client::new(settings.token.as_str(), Handler).expect("Connection to Discord");

    // Build the framework setup
    let framework = StandardFramework::new()
        .simple_bucket("simple", 2)
        .simple_bucket("moderate", 6)
        .configure(|c| {
            c.allow_dm(true)
                .on_mention(true)
                .allow_whitespace(false)
                .depth(2)
                .owners(settings.admins.into_iter().map(UserId).collect())
                .blocked_users(settings.blocked_users.into_iter().map(UserId).collect())
                .disabled_commands(settings.disabled_commands.into_iter().collect())
                .prefixes(vec!["%", "!", "~", "?"])
                .case_insensitivity(true)
        })
        .help(help_commands::with_embeds)
        .after(|_, message, command, _error| {
            debug!(
                "Received command {} from @{}#{}",
                command, message.author.name, message.author.discriminator
            );
        })
        .group("misc", |g| {
            g.bucket("simple")
                .cmd("ping", commands::misc::ping)
                .cmd("trump", commands::misc::trump::command)
        })
        .group("repl", |g| {
            g.bucket("moderate")
                .prefix("repl")
                .cmd("rust", commands::repl::rust::command)
                .cmd("go", commands::repl::go::command)
        });
    client.with_framework(framework);

    if let Err(e) = client.start_autosharded() {
        println!("Uh-oh, Dolan malfunctioned: {:#?}", e);
    }
}
