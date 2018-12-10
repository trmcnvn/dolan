use serenity::command;

pub mod trump;

command!(ping(_context, message) {
    if let Err(e) = message.reply("Pong!") {
        println!("Error sending pong: {:?}", e);
    }
});
