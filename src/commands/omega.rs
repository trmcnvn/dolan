use crate::utils::get_emoji_by_name;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::utils::MessageBuilder;

lazy_static! {
    static ref OMEGA: Regex = Regex::new(r"[oO0]").expect("Regexp to compile");
}

command!(cmd(_ctx, message, args) {
    let target = args.rest();
    let result = OMEGA.replace_all(target, |_captures: &regex::Captures| {
        get_emoji_by_name(message.guild_id, "OMEGALUL")
    });
    let response = MessageBuilder::new().push(result).build();
    message.channel_id.say(&response)?;
});
