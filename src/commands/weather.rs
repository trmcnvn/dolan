use log::debug;
use reqwest::{self, header::USER_AGENT};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
fn weather(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let location = args.current().unwrap_or_default();
    let endpoint = format!("https://wttr.in/{}?0qT&lang=en", location);

    let client = reqwest::Client::new();
    let request = client.get(&endpoint).header(USER_AGENT, "curl");
    let mut res = match request.send() {
        Ok(res) => res,
        Err(e) => {
            debug!("Error: {:#?}", e);
            msg.reply(&ctx, "There was an issue getting the weather...")?;
            return Ok(());
        }
    };

    let text = match res.text() {
        Ok(text) => text,
        Err(e) => {
            debug!("Error: {:#?}", e);
            msg.reply(&ctx, "There was an issue with the weather...")?;
            return Ok(());
        }
    };

    if text.len() >= 2000 {
        debug!("Message was too long, converting to image...");
        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| e.image(format!("https://wttr.in/{}_0q_lang=en.png", location)))
        })?;
    } else {
        let message_builder = MessageBuilder::new().push_codeblock(text, None).build();
        msg.channel_id.say(&ctx, &message_builder)?;
    }
    Ok(())
}
