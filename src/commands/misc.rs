use serenity::command;
use log::debug;

pub mod trump;

command!(ping(_context, message) {
    if let Err(e) = message.reply("Pong!") {
        debug!("Error sending pong: {:?}", e);
    }
});
