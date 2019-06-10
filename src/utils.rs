use serenity::model::id::GuildId;

pub fn get_emoji_by_name(guild_id: Option<GuildId>, name: &str) -> String {
    if let Some(guild_id) = guild_id {
        if let Some(guild) = guild_id.to_guild_cached() {
            let emojis = &guild.read().emojis;
            for emoji in emojis.values() {
                if emoji.name == name {
                    return format!("<:{}:{}>", name, emoji.id.as_u64());
                }
            }
        }
    }
    String::from("<:grey_question:582795707401109506>")
}

/// Taken from `maplit` crate to work with hashbrown.
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
