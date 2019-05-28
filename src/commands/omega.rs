use crate::utils::get_emoji_by_name;
use serenity::utils::MessageBuilder;

command!(cmd(_ctx, message, args) {
    let target = args.rest();
    let emoji = get_emoji_by_name(message.guild_id, "OMEGALUL");
    let result = target.replace("o", &emoji).replace("O", &emoji);
    let response = MessageBuilder::new().push(result).build();
    message.channel_id.say(&response)?;
});
