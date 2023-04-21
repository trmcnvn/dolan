use chrono::{DateTime, NaiveDate, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn diablo(ctx: &Context, msg: &Message) -> CommandResult {
    // Diablo 4 stress test
    let st_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 5, 12).and_hms(19, 0, 0), Utc);
    // Diablo 4 launch date
    let rl_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 6, 2).and_hms(16, 0, 0), Utc);
    // format message
    let message = MessageBuilder::new()
        .push_line(format!(
            "**Server Slam**: {}",
            HumanTime::from(st_dt).to_text_en(Accuracy::Precise, Tense::Present)
        ))
        .push_line(format!(
            "**Release**: {}",
            HumanTime::from(rl_dt).to_text_en(Accuracy::Precise, Tense::Present)
        ))
        .build();
    msg.channel_id.say(&ctx, message).await?;
    Ok(())
}
