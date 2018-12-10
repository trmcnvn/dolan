use serenity::command;

command!(ping(_context, message) {
    if let Err(e) = message.reply("Pong!") {
        println!("Error sending pong: {:?}", e);
    }
});
