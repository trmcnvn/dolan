use log::debug;
use reqwest::{self, header::USER_AGENT};
use serenity::framework::standard::{Args, Command, CommandError as Error, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use std::sync::Arc;

pub struct Weather;
impl Command for Weather {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            desc: Some("Gets the weather information for the area specified".into()),
            usage: Some("?weather <location/coordinates/IATA airport code>".into()),
            min_args: Some(1),
            ..CommandOptions::default()
        })
    }

    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), Error> {
        let location = args.current().unwrap_or_default();
        let endpoint = format!("https://wttr.in/{}?0qT&lang=en", location);

        let client = reqwest::Client::new();
        let request = client.get(&endpoint).header(USER_AGENT, "curl");
        let mut res = match request.send() {
            Ok(res) => res,
            Err(e) => {
                debug!("Error: {:#?}", e);
                message.reply("There was an issue getting the weather...")?;
                return Ok(());
            }
        };

        let text = match res.text() {
            Ok(text) => text,
            Err(e) => {
                debug!("Error: {:#?}", e);
                message.reply("There was an issue with the weather...")?;
                return Ok(());
            }
        };

        if text.len() >= 2000 {
            debug!("Message was too long, converting to image...");
            message.channel_id.send_message(|m| {
                m.embed(|e| e.image(format!("https://wttr.in/{}_0q_lang=en.png", location)))
            })?;
        } else {
            let message_builder = MessageBuilder::new().push_codeblock(text, None).build();
            message.channel_id.say(&message_builder)?;
        }
        Ok(())
    }
}
