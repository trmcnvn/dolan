use crate::utils::get_emoji_by_name;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx, get_emoji_by_name(ctx, msg.guild_id, "Pog"))
        .await?;
    Ok(())
}
