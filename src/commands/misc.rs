use log::debug;
use serenity::command;

pub mod trump;

command!(ping(_context, message) {
    if let Err(e) = message.reply("Pong!") {
        debug!("Error sending pong: {:?}", e);
    }
});
