use serenity::cache::Cache;
use serenity::model::id::GuildId;

// Find an emoji on a Discord "guild" by its name
pub fn get_emoji_by_name(
    cache: impl AsRef<Cache>,
    guild_id: Option<GuildId>,
    name: &str,
) -> String {
    if let Some(guild_id) = guild_id {
        if let Some(guild) = guild_id.to_guild_cached(&cache) {
            let emojis = &guild.emojis;
            for emoji in emojis.values() {
                if emoji.name == name {
                    return format!("<:{}:{}>", name, emoji.id.as_u64());
                }
            }
        }
    }
    String::from("<:grey_question:582795707401109506>")
}
