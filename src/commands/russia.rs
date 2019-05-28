use serenity::utils::MessageBuilder;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_MAP: HashMap<char, char> = hashmap!{
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


command!(cmd(_ctx, message, args) {
    let mut target = String::new();
    for c in args.rest().chars() {
        if LANG_MAP.contains_key(&c) {
            target.push(*LANG_MAP.get(&c).unwrap());
        } else {
            target.push(c);
        }
    }
    let response = MessageBuilder::new().push(target).build();
    message.channel_id.say(&response)?;
});
