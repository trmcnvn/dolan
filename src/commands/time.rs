use log::debug;
use reqwest;
use select::document::Document;
use select::predicate::Attr;
use serenity::utils::MessageBuilder;

command!(cmd(_ctx, message, args) {
    let timezones: Vec<&str> = args.rest().split(',').collect();
        let mut times: Vec<String> = Vec::with_capacity(timezones.len());
        for timezone in timezones {
            let validated_timezone = timezone.replace(|c: char| !c.is_ascii(), "");
            if validated_timezone.trim().is_empty() {
                continue;
            }

            debug!("Timezone: {}", validated_timezone);
            let endpoint = format!("https://time.is/{}", validated_timezone);
            let response = reqwest::get(&endpoint).unwrap();
            let document = Document::from_read(response).unwrap();
            let time = document.find(Attr("id", "twd")).next().unwrap().text();
            let human_timezone = document
                .find(Attr("id", "msgdiv"))
                .next()
                .unwrap()
                .first_child()
                .unwrap()
                .text();
            if human_timezone.trim().is_empty() {
                message.reply(&format!(
                    "{} isn't valid... yikes... you really should learn your timezones.",
                    validated_timezone
                ))?;
                continue;
            }

            times.push(format!("{}: {}", human_timezone.trim(), time.trim()));
        }
        if !times.is_empty() {
            let message_builder = MessageBuilder::new()
                .mention(&message.author)
                .push(" ")
                .push_codeblock(times.join("\n"), None)
                .build();
            message.channel_id.say(&message_builder)?;
        }
});
