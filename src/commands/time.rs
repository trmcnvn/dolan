use select::document::Document;
use select::predicate::Name;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

/// Melbourne, Australia; Kyiv, Ukraine
#[command]
fn time(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let timezones: Vec<&str> = args.rest().split(';').collect();
    let mut times: Vec<String> = Vec::with_capacity(timezones.len());
    for timezone in timezones {
        let validated_timezone = timezone.replace(|c: char| !c.is_ascii(), "");
        if validated_timezone.trim().is_empty() {
            continue;
        }

        let endpoint = format!("https://time.is/{}", validated_timezone);
        let response = reqwest::blocking::get(&endpoint)?;

        // If timezone is inacurate, the title will just be the local time for where this bot is running
        let document = Document::from_read(response)?;
        if let Some(time) = document.find(Name("time")).next() {
            times.push(format!("{}: {}", timezone, time.text()));
        }
    }

    if !times.is_empty() {
        let message_builder = MessageBuilder::new()
            .mention(&msg.author)
            .push(" ")
            .push_codeblock(times.join("\n"), None)
            .build();
        msg.channel_id.say(&ctx, &message_builder)?;
    }

    Ok(())
}
