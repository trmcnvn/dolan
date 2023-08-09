use reqwest::{self, header::USER_AGENT};
use select::document::Document;
use select::predicate::Name;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use urlencoding::encode;

/// Melbourne, Australia; Kyiv, Ukraine
#[command]
async fn time(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let client = reqwest::Client::new();
    let timezones: Vec<&str> = args.rest().split(';').collect();
    let mut times: Vec<String> = Vec::with_capacity(timezones.len());
    for timezone in timezones {
        let validated_timezone = encode(timezone);
        if validated_timezone.trim().is_empty() {
            continue;
        }

        let endpoint = format!("https://time.is/{}", validated_timezone);
        let response = client
            .get(&endpoint)
            .header(USER_AGENT, "Dolan/1.0")
            .send()
            .await?
            .text()
            .await?;

        // If timezone is inacurate, the title will just be the local time for where this bot is running
        let document = Document::from(response.as_str());
        if let Some(time) = document.find(Name("time")).next() {
            times.push(format!("{}: {}", timezone, time.text()));
        }
    }

    if !times.is_empty() {
        let message_builder = MessageBuilder::new()
            .push_codeblock(times.join("\n"), None)
            .build();
        msg.channel_id.say(&ctx, &message_builder).await?;
    }

    Ok(())
}
