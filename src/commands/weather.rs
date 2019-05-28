use log::debug;
use reqwest::{self, header::USER_AGENT};
use serenity::utils::MessageBuilder;

command!(cmd(_ctx, message, args) {
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
});
