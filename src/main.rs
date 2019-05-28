#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]

#[macro_use]
extern crate serenity;

use crate::settings::SETTINGS;
use env_logger::{Builder, WriteStyle};
use log::{debug, info, LevelFilter};
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{help_commands, StandardFramework};
use serenity::model::gateway::Ready;

#[macro_use]
mod utils;
mod commands;
mod settings;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, _ready: Ready) {
        context.set_game("?help");
        info!("Dolan is connected...");
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
        .filter(Some("dolan"), level)
        .write_style(WriteStyle::Always)
        .init();

    // Initialize connection to Discord via token
    let mut client = Client::new(settings.token.as_str(), Handler).expect("Connection to Discord");

    // Build the framework setup
    let framework = StandardFramework::new()
        .simple_bucket("simple", 2)
        .configure(|c| {
            c.allow_dm(true)
                .on_mention(true)
                .allow_whitespace(false)
                .depth(2)
                .prefix("?")
                .case_insensitivity(true)
        })
        .help(help_commands::with_embeds)
        .after(|_, message, command, _error| {
            debug!(
                "Received command {} from @{}#{}",
                command, message.author.name, message.author.discriminator
            );
        })
        .command("ping", |c| c.cmd(commands::ping::cmd))
        .command("repl", |c| c.cmd(commands::repl::cmd))
        .command("trump", |c| c.cmd(commands::trump::cmd))
        .command("weather", |c| c.cmd(commands::weather::cmd))
        .command("time", |c| c.cmd(commands::time::cmd))
        .command("omega", |c| c.cmd(commands::omega::cmd))
        .command("russia", |c| c.cmd(commands::russia::cmd));
    client.with_framework(framework);

    if let Err(e) = client.start_autosharded() {
        println!("Uh-oh, Dolan malfunctioned: {:#?}", e);
    }
}
