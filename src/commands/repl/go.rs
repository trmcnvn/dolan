use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use reqwest;
use serde_derive::{Deserialize, Serialize};
use serenity::command;
use serenity::utils::MessageBuilder;

lazy_static! {
    static ref CODE: Regex =
        Regex::new(r".+\n+\x60{3}(go)\n([\s\S]*?)\x60{3}").expect("Compile regex");
}

#[derive(Debug, Serialize)]
pub struct Request {
    version: u32,
    body: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub errors: String,
    pub events: Option<Vec<Event>>,
    pub status: u32,
    pub tests_failed: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub delay: u32,
    pub kind: String,
    pub message: String,
}

// ?repl go
command!(command(_context, message) {
    let caps = if let Some(caps) = CODE.captures(&message.content) {
        caps
    } else {
        message.reply("Couldn't parse your code. Make sure you wrap it in codeblocks with ```go")?;
        return Ok(());
    };

    // build request payload
    let payload = Request {
        version: 2,
        body: caps[2].into()
    };
    debug!("Go payload: {:?}", payload);

    // make request to the playground
    let client = reqwest::Client::new();
    let mut res = match client.post("https://play.golang.org/compile").json(&payload).send() {
        Ok(res) => res,
        Err(e) => {
            debug!("Error: {:#?}", e);
            message.reply("There was an issue sending the code to the REPL.")?;
            return Ok(());
        }
    };

    // deserialize json response into struct
    let json: Response = match res.json() {
        Ok(json) => json,
        Err(e) => {
            debug!("Error: {:#?}", e);
            message.reply("There was an issue with the response from the REPL.")?;
            return Ok(());
        }
    };
    debug!("Go response: {:?}", json);

    // reply to user
    let message_builder = match json.events {
        Some(events) => {
            let output: String = events.into_iter().map(|e| e.message).collect::<Vec<String>>().join("\n");
            MessageBuilder::new()
                .mention(&message.author)
                .push(" ")
                .push("here's the output:")
                .push_codeblock(output, Some("go"))
                .build()
        }
        None => MessageBuilder::new()
            .mention(&message.author)
            .push(" ")
            .push("your compilation failed... yikes...")
            .push_codeblock(json.errors, Some("go"))
            .build()
    };
    message.channel_id.say(&message_builder)?;
});
