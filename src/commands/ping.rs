use serenity::framework::standard::{Args, Command, CommandError as Error, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::sync::Arc;

pub struct Ping;
impl Command for Ping {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            desc: Some("Replies to the PING with a PONG".into()),
            usage: Some("?ping".into()),
            max_args: Some(0),
            ..CommandOptions::default()
        })
    }

    fn execute(&self, _context: &mut Context, message: &Message, _args: Args) -> Result<(), Error> {
        message.reply("Pong!")?;
        Ok(())
    }
}
