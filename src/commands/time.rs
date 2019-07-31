use reqwest;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
fn time(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let timezones: Vec<&str> = args.rest().split(',').collect();
    let mut times: Vec<String> = Vec::with_capacity(timezones.len());
    for timezone in timezones {
        let validated_timezone = timezone.replace(|c: char| !c.is_ascii(), "");
        if validated_timezone.trim().is_empty() {
            continue;
        }
        let endpoint = format!("https://time.is/{}", validated_timezone);
        let response = reqwest::get(&endpoint).unwrap();
        let document = Document::from_read(response).unwrap();
        let time = document.find(Attr("id", "twd")).next().unwrap().text();
        let human_timezone = document
            .find(Attr("id", "msgdiv"))
            .next()
            .unwrap()
            .first_child()
            .unwrap()
            .text();
        if human_timezone.trim().is_empty() {
            msg.reply(
                &ctx,
                &format!(
                    "{} isn't valid... yikes... you really should learn your timezones.",
                    validated_timezone
                ),
            )?;
            continue;
        }
        times.push(format!("{}: {}", human_timezone.trim(), time.trim()));
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
