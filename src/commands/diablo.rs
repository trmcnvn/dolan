use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_humanize::{HumanTime, Accuracy, Tense};

#[command]
async fn diablo(ctx: &Context, msg: &Message) -> CommandResult {
    // Diablo 4 release date
    let dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 3, 17).and_hms(16, 0, 0), Utc);
    // Convert to humanized format
    let ht = HumanTime::from(dt);
    msg.channel_id
        .say(&ctx, ht.to_text_en(Accuracy::Precise, Tense::Present))
        .await?;
    Ok(())
}
