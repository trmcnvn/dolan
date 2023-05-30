use chrono::{DateTime, NaiveDate, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn diablo(ctx: &Context, msg: &Message) -> CommandResult {
    // Diablo 4 preload date
    let pl_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 5, 30).and_hms(23, 0, 0), Utc);
    // Diablo 4 launch date
    let rl_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 6, 1).and_hms(23, 0, 0), Utc);
    // format messag
    let message = MessageBuilder::new()
        .push_line(format!(
            "**Preload**: {}",
            HumanTime::from(pl_dt).to_text_en(Accuracy::Precise, Tense::Future)
        ))
        .push_line(format!(
            "**Release**: {}",
            HumanTime::from(rl_dt).to_text_en(Accuracy::Precise, Tense::Future)
        ))
        .build();
    msg.channel_id.say(&ctx, message).await?;
    Ok(())
}
