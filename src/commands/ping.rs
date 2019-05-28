use crate::utils::get_emoji_by_name;
use serenity::utils::MessageBuilder;

command!(cmd(_ctx, message) {
    let response = MessageBuilder::new().push(get_emoji_by_name(message.guild_id, "Pog")).build();
    message.channel_id.say(&response)?;
});
