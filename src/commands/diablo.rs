use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_humanize::{HumanTime, Accuracy, Tense};
use serenity::utils::MessageBuilder;

#[command]
async fn diablo(ctx: &Context, msg: &Message) -> CommandResult {
    // Diablo 4 preload date
    let pl_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 3, 15).and_hms(16, 0, 0), Utc);
    // Diablo 4 beta date
    let cb_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 3, 17).and_hms(16, 0, 0), Utc);
    // Diablo 4 open beta date
    let ob_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 3, 24).and_hms(16, 0, 0), Utc);
    // Diablo 4 launch date
    let rl_dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 6, 6).and_hms(16, 0, 0), Utc);
    // format message
    let message = MessageBuilder::new()
        .push_line(format!("**Preload**: {}", HumanTime::from(pl_dt).to_text_en(Accuracy::Precise, Tense::Present)))
        .push_line(format!("**Closed Beta**: {}", HumanTime::from(cb_dt).to_text_en(Accuracy::Precise, Tense::Present)))
        .push_line(format!("**Open Beta**: {}", HumanTime::from(ob_dt).to_text_en(Accuracy::Precise, Tense::Present)))
        .push_line(format!("**Release**: {}", HumanTime::from(rl_dt).to_text_en(Accuracy::Precise, Tense::Present)))
        .build();
    msg.channel_id
        .say(&ctx, message)
        .await?;
    Ok(())
}
