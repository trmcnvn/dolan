use log::debug;
use reqwest::{self, header::USER_AGENT};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use urlencoding::encode;

#[command]
async fn weather(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let client = reqwest::blocking::Client::new();
    let locations: Vec<&str> = args.rest().split(';').collect();
    let mut messages = Vec::with_capacity(locations.len());

    for location in locations {
        let valid_location = encode(location);
        if valid_location.trim().is_empty() {
            continue;
        }

        let endpoint = format!("https://wttr.in/{}?m0qT&lang=en", valid_location);
        let request = client.get(&endpoint).header(USER_AGENT, "curl");
        if let Ok(response) = request.send() {
            if let Ok(text) = response.text() {
                messages.push((valid_location, text));
            }
        }
    }

    for (location, message) in messages {
        if message.len() >= 2000 {
            debug!("Message was too long, converting to image...");
            msg.channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| e.image(format!("https://wttr.in/{}_m0q_lang=en.png", location)))
                })
                .await?;
        } else {
            let message_builder = MessageBuilder::new().push_codeblock(message, None).build();
            msg.channel_id.say(&ctx, &message_builder).await?;
        }
    }

    Ok(())
}
