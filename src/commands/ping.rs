use crate::utils::get_emoji_by_name;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx, get_emoji_by_name(&ctx, msg.guild_id, "Pog"))?;
    Ok(())
}
