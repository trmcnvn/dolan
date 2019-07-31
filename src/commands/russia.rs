use lazy_static::lazy_static;
use maplit::hashmap;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_MAP: HashMap<char, char> = hashmap! {
        'a' => 'д',
        'b' => 'б',
        'e' => 'ё',
        'h' => 'н',
        'm' => 'м',
        'n' => 'и',
        'r' => 'г',
        'u' => 'ц',
        'x' => 'ж',
        'A' => 'Д',
        'B' => 'Б',
        'E' => 'Ё',
        'H' => 'Н',
        'M' => 'М',
        'N' => 'И',
        'R' => 'Я',
        'U' => 'Ц',
        'X' => 'Ж',
    };
}

#[command]
fn russia(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut target = String::new();
    for c in args.rest().chars() {
        if LANG_MAP.contains_key(&c) {
            target.push(*LANG_MAP.get(&c).unwrap());
        } else {
            target.push(c);
        }
    }
    msg.channel_id.say(&ctx, target)?;
    Ok(())
}
