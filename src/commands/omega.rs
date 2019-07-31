use crate::utils::get_emoji_by_name;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

lazy_static! {
    static ref OMEGA: Regex = Regex::new(r"[oO0]").expect("Regexp to compile");
}

#[command]
fn omega(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let target = args.rest();
    let result = OMEGA.replace_all(target, |_captures: &regex::Captures| {
        get_emoji_by_name(&ctx, msg.guild_id, "OMEGALUL")
    });
    msg.channel_id.say(&ctx, result)?;
    Ok(())
}
